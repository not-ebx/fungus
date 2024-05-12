use crate::session::client_session::ClientSession;
use fungus_database::models::user::User;
use fungus_packet_utils::in_packet::InPacket;
use fungus_packet_utils::out_packet::OutPacket;
use fungus_packet_utils::packet_errors::PacketError;
use fungus_utils::constants::server_constants::ALLOW_AUTO_REGISTER;
use fungus_utils::enums::login_type::LoginType;
use std::io::Read;
use crate::packets::login_packets::login_packets::{on_check_password_result, on_send_recommended_world_message, on_send_world_information_end,on_send_world_status};
use crate::server::server::SERVER_INSTANCE;

pub async fn handle_check_login_auth_info(
    session: &mut ClientSession,
    in_packet: &mut InPacket,
) -> Result<(), PacketError> {
    let username: String = in_packet.read_string()?.to_string();
    let password: String = in_packet.read_string()?.to_string();
    let mut machine_id = vec![16];
    in_packet.read_exact(&mut machine_id)?;

    let user_result = User::get_user_by_username(username.clone());

    let packet = match user_result {
        Ok(user) => {
            // TODO - Check for bans and stuff.
            if user.check_password(password) {
                let out_packet = on_check_password_result(Option::from(&user), LoginType::Success);
                session.user = Some(user);
                session.machine_id = std::str::from_utf8(machine_id.as_slice()).unwrap_or("").to_string();
                let user_ref = session.get_user_ref();
                SERVER_INSTANCE.write().await.add_user(user_ref.unwrap());
                out_packet
            } else {
                on_check_password_result(
                    Option::from(&user),
                    LoginType::IncorrectPassword
                )
            }
        }
        _ => {
            if ALLOW_AUTO_REGISTER {
                let new_user = User::insert_new_user(username.clone(), password.clone()).unwrap();
                let out_packet = on_check_password_result(
                    Option::from(&new_user),
                    LoginType::Success
                );

                session.user = Some(new_user);
                session.machine_id = std::str::from_utf8(machine_id.as_slice()).unwrap_or("").to_string();
                let user_ref = session.get_user_ref();
                SERVER_INSTANCE.write().await.add_user(user_ref.unwrap());
                out_packet
            } else {
                on_check_password_result(None, LoginType::NotRegistered)
            }
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


    Ok(())
}