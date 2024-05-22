use std::ops::Not;
use fungus_game::entities::character::Character;
use crate::session::client_session::ClientSession;
use fungus_game::entities::user::User;
use fungus_game::errors::service_errors::ServiceError;
use fungus_packet_utils::in_packet::InPacket;
use fungus_packet_utils::out_packet::OutPacket;
use fungus_packet_utils::packet_errors::PacketError;
use fungus_utils::enums::character_id_result::CharacterIDResult;
use fungus_utils::enums::login_type::LoginType;
use fungus_utils::enums::server_status::ServerStatus;
use crate::packets::login_packets::login_packets::{on_check_duplicated_id_result, on_check_password_result, on_select_world_result, on_send_account_info, on_send_recommended_world_message, on_send_world_information_end, on_send_world_status};

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
    let user_service = session.service_registry.clone().get_user_service();
    let user_result = user_service.try_login(
        username.clone(),
        password.clone()
    ).await;

    let login_type = match user_result {
        Ok(user) => {
            // TODO - Check for bans and stuff.
            logged_user = Some(user);
            LoginType::Success
        }
        Err(ServiceError::NotFound) => {
            LoginType::NotRegistered
        }
        Err(ServiceError::InvalidDetails) => {
            LoginType::IncorrectPassword
        }
        _ => {
            LoginType::Unknown
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
    let server_instance = session.server_instance.clone();
    let worlds = server_instance.get_worlds();

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

    let world_search = {
        let server_instance = session.server_instance.clone();
        let worlds_lock = server_instance.get_worlds();
        let worlds = worlds_lock.read().await;
        worlds.iter().find(|&w| w.id == world_id).cloned()
    };

    let world_status = match world_search {
        None => {
            ServerStatus::Busy as u8
        }
        Some(world) => {
            world.get_status() as u8
        }
    };

    session.send_packet(
        &on_send_world_status(world_status).await
    ).await?;

    Ok(())
}

pub async fn handle_select_world(session: &mut ClientSession, in_packet: &mut InPacket) -> Result<(), PacketError> {
    let _ = in_packet.read_byte()?;
    let world_id = in_packet.read_byte()?;
    let channel = in_packet.read_byte()? + 1;

    // Check if the user has an Account created in that world, else, create a new one.
    {
        let account_service = session.service_registry.get_account_service();
        let user = session.user.as_mut().unwrap();
        let account = account_service.get_account(user.id, world_id as i16).await;
        if let Err(_) = account {
            return Err(PacketError::CommunicationError())
        }
        session.account = Some(account.unwrap());
        session.world_id = world_id as i16;
    }
    // Better top handle channel stuff form the server instance itself.
    //session.channel = SERVER_INSTANCE.read().await.get_channel(world_id as i16, channel as i32).await
    session.send_packet(&on_send_account_info(session.user.as_ref().unwrap()).await).await.unwrap();
    session.send_packet(&on_select_world_result(session.user.as_ref().unwrap(), session.account.as_ref().unwrap()).await).await.unwrap();
    Ok(())

}

pub async fn handle_check_duplicate_id(session: &mut ClientSession, in_packet: &mut InPacket) -> Result<(), PacketError> {
    let character_name = in_packet.read_string()?;
    let mut check_result: CharacterIDResult = CharacterIDResult::Invalid;
    if character_name.len() > 13 && character_name.len() < 4 {
        check_result = CharacterIDResult::Invalid;
    } else if session.service_registry.get_character_service().is_duplicated_id(character_name).await.not() {
        check_result = CharacterIDResult::Available;
    } else {
        check_result = CharacterIDResult::InUse;
    }

    session.send_packet(
        &on_check_duplicated_id_result(character_name, check_result).await
    ).await
}