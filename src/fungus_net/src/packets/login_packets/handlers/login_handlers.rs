use crate::session::client_session::ClientSession;
use fungus_database::models::user::User;
use fungus_packet_utils::in_packet::InPacket;
use fungus_packet_utils::out_packet::OutPacket;
use fungus_packet_utils::packet_errors::PacketError;
use fungus_utils::constants::server_constants::ALLOW_AUTO_REGISTER;
use fungus_utils::enums::login_type::LoginType;
use crate::packets::login_packets::login_packets::{on_check_password_result, on_select_world_result, on_send_account_info, on_send_recommended_world_message, on_send_world_information_end, on_send_world_status};
use crate::server::server::SERVER_INSTANCE;

pub async fn handle_check_login_auth_info(
    session: &mut ClientSession,
    in_packet: &mut InPacket,
) -> Result<(), PacketError> {
    let username: String = in_packet.read_string()?.to_string();
    let password: String = in_packet.read_string()?.to_string();
    let mut machine_id = vec![16];
    in_packet.read_exact(&mut machine_id)?;

    // things used to pass to session
    let mut logged_user: Option<User> = None;

    let user_result = User::get_user_by_username(username.clone()).await;

    let login_type = match user_result {
        Ok(user) => {
            // TODO - Check for bans and stuff.
            if user.check_password(password) {
                logged_user = Some(user);
                LoginType::Success
            } else {
                LoginType::IncorrectPassword
            }
        }
        _ => {
            if ALLOW_AUTO_REGISTER {
                let new_user = User::insert_new_user(username.clone(), password.clone()).await.expect("Could NOT autoregister");
                logged_user = Some(new_user);
                LoginType::Success
            } else {
                LoginType::NotRegistered
            }
        }
    };

    let packet = match login_type {
        LoginType::Success => {
            // We se the values on the session then send the packet.
            session.set_user(logged_user.unwrap());
            session.machine_id = std::str::from_utf8(&*machine_id).unwrap().to_string();
            let user_ref = session.get_user_ref();
            on_check_password_result(user_ref, login_type)
        }
        _ => {
            on_check_password_result(None, login_type)
        }
    };

    match packet {
        None => {
            Err(PacketError::UnimplementedPacket("No response implemented".to_string()))
        }
        Some(out_packet) => {
            session.send_packet(&out_packet).await?;
            Ok(())
        }
    }
}


pub async fn handle_world_list_request(session: &mut ClientSession, in_packet: InPacket) -> Result<(), PacketError> {
    let worlds = SERVER_INSTANCE.read().await.get_worlds();

    for world in worlds.read().await.iter() {
        let world_info_packet = OutPacket::from(world.get_info_as_packet().await);
        session.send_packet(
            &world_info_packet
        ).await?;
    }
    session.send_packet(
        &on_send_world_information_end()
    ).await?;

    session.send_packet(
        &on_send_recommended_world_message(0, String::from("Idk"))
    ).await?;

    Ok(())
}

pub async fn handle_world_status_request(session: &mut ClientSession, in_packet: &mut InPacket) -> Result<(), PacketError>{
    let world_id: i32 = in_packet.read_byte()? as i32;
    session.send_packet(
        &on_send_world_status(world_id).await
    ).await?;

    Ok(())
}

pub async fn handle_select_world(session: &mut ClientSession, in_packet: &mut InPacket) -> Result<(), PacketError> {
    let _ = in_packet.read_byte()?;
    let world_id = in_packet.read_byte()?;
    let channel = in_packet.read_byte()? + 1;

    // Check if the user has an Account created in that world, else, create a new one.
    {
        let mut user = session.user.as_mut().unwrap();
        let acc = user.get_account(world_id as i16).await.unwrap();
        session.account = Some(acc);
        session.world_id = world_id as i16;
    }
    // Better top handle channel stuff form the server instance itself.
    //session.channel = SERVER_INSTANCE.read().await.get_channel(world_id as i16, channel as i32).await
    session.send_packet(&on_send_account_info(session.user.as_ref().unwrap()).await).await.unwrap();
    session.send_packet(&on_select_world_result(session.user.as_ref().unwrap(), session.account.as_ref().unwrap()).await).await.unwrap();
    Ok(())

}