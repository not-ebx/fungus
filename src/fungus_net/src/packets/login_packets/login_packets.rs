use std::future::Future;
use fungus_database::models::user::User;
use fungus_packet_utils::out_headers::OutHeader;
use fungus_packet_utils::out_packet::OutPacket;
use fungus_utils::constants::server_constants::{
    ALLOW_AUTO_REGISTER, LOCALE, MINOR_VERSION, VERSION,
};
use fungus_utils::enums::login_type::LoginType;
use fungus_utils::types::fungus_time::FungusTime;
use rand::{Rng, RngCore};
use fungus_database::models::account::Account;
use fungus_utils::enums::character_id_result::CharacterIDResult;
use fungus_utils::enums::server_status::ServerStatus;
use fungus_world::world::World;
use crate::server::server::SERVER_INSTANCE;

pub fn on_send_ping() -> OutPacket {
    OutPacket::new(OutHeader::AliveReq)
}

pub fn on_send_connect(siv: &[u8], riv: &[u8]) -> OutPacket {
    // Length = 13 + "1"
    let mut out_packet: OutPacket = OutPacket::default();

    out_packet.write_short(14); // Size of packet
    out_packet.write_short(VERSION);
    out_packet.write_string(MINOR_VERSION.to_string());
    out_packet.write_bytes(&riv.to_vec());
    out_packet.write_bytes(&siv.to_vec());
    out_packet.write_byte(LOCALE);

    out_packet
}

pub fn on_check_password_result(user: Option<&User>, success_code: LoginType) -> Option<OutPacket> {
    match success_code {
        LoginType::Success => {
            let found_user = user.unwrap();
            Some(on_success_login(found_user))
        }
        LoginType::NotRegistered => {
            // Send not registered packet
            // TODO fix this lmao
            None
        }

        LoginType::IncorrectPassword => {
            // Send not registered packet
            // TODO fix this lmao
            None
        }
        /*
        LoginType::TempBlocked => {}
        LoginType::Blocked => {}
        LoginType::Abandoned => {}

        LoginType::DBFail => {}
        LoginType::AlreadyConnected => {}
        LoginType::NotConnectableWorld => {}
         */
        _ => None,
    }
}

// This has to own the user.
pub fn on_success_login(user: &User) -> OutPacket {
    let mut out_packet: OutPacket = OutPacket::new(OutHeader::CheckPasswordResult);

    // A byte idk
    // Success code
    out_packet.write_byte(LoginType::Success as u8);
    out_packet.write_byte(0);
    out_packet.write_int(0);
    out_packet.write_int(user.id);
    out_packet.write_byte(user.gender as u8);
    // Todo gotta handle this better, for the meantime, admin account will work as this
    out_packet.write_bool(user.account_type > 0); // Something about gm..?
    out_packet.write_short(0); // Gm level i think
    out_packet.write_bool(user.account_type > 0); // Something about admin account idk
    out_packet.write_string(user.username.clone());
    out_packet.write_byte(3); // 3 for new accds .. ?
    out_packet.write_byte(0); // quiet ban
    out_packet.write_long(0); // quiet ban time
    out_packet.write_byte(1); // idk?

    // Get the time
    let ms_time = FungusTime::from(user.created_at.clone());

    out_packet.write(ms_time);

    // Something to select the world
    out_packet.write_int(4);
    out_packet.write_byte(1); // Pin Disabled
    out_packet.write_byte(2); // Pic Disabled, 2

    let mut rng = rand::thread_rng();
    let random_long = rng.next_u64() as i64; // TODO gotta create a randomizer :)
    out_packet.write_long(random_long);

    out_packet
}

pub fn on_send_world_information_end() -> OutPacket{
    let mut out_packet = OutPacket::new(OutHeader::WorldInformation);
    out_packet.write_int(255);
    out_packet
}

// TODO Implement this, for the meantime no message :)
pub fn on_send_recommended_world_message(recommended_world_id: i32, message: String) -> OutPacket {
    let mut out_packet = OutPacket::new(OutHeader::RecommendedWorldMessage);
    out_packet.write_bool(message.len() > 0); // is message empty?
    out_packet.write_int(recommended_world_id); // World id
    out_packet.write_string(message);

    out_packet
}

pub async fn on_send_world_status(world_id: i32) -> OutPacket {
    let mut out_packet: OutPacket = OutPacket::new(OutHeader::CheckUserLimitResult);

    let world_search = {
        let server_instance = SERVER_INSTANCE.write().await;
        let worlds_lock = server_instance.get_worlds();
        let worlds = worlds_lock.read().await;
        worlds.iter().find(|&w| w.id == world_id).cloned() // Assuming the data inside is cloneable
    };

    match world_search {
        None => {
            out_packet.write_byte(ServerStatus::Busy as u8);
        }
        Some(world) => {
            out_packet.write_byte(world.get_status() as u8);
        }
    }

    out_packet.write_byte(0);
    out_packet
}

pub async fn on_send_account_info(user: &User) -> OutPacket {
    let mut out_packet = OutPacket::new(OutHeader::AccountInfoResult);

    out_packet.write_byte(0); //Operation
    out_packet.write_int(user.id);
    out_packet.write_byte(user.gender as u8);
    // Todo gotta handle this better, for the meantime, admin account will work as this
    out_packet.write_bool(user.account_type > 0); // Something about gm..?
    out_packet.write_short(2); // Gm level i think
    out_packet.write_bool(user.account_type > 0); // Something about admin account idk
    out_packet.write_string(user.username.clone());
    out_packet.write_byte(3); // 3 for new accds .. ?
    out_packet.write_byte(0); // quiet ban
    out_packet.write_long(0); // quiet ban time

    // Get the time
    let ms_time = FungusTime::from(user.created_at.clone());

    out_packet.write(ms_time);

    // Something to select the world
    out_packet.write_int(4);
    let mut rng = rand::thread_rng();
    let random_long = rng.next_u64() as i64; // TODO gotta create a randomizer :)
    out_packet.write_long(random_long);
    out_packet.write_byte(0);

    out_packet

}

pub async fn on_select_world_result(user: &User, account: &Account) -> OutPacket {
    let mut out_packet = OutPacket::new(OutHeader::SelectWorldResult);
    let characters = account.get_characters().await.unwrap();

    out_packet.write_byte(0); // Success code
    //out_packet.write_byte(characters.len() as u8);
    out_packet.write_byte(0);
    /*for character in characters.iter() {
        // TODO Encode character data.
        //out_packet.write_byte(0); // Family stuff.
    }*/

    out_packet.write_byte(2); // bLoginOpt
    out_packet.write_byte(1);
    out_packet.write_int(account.character_slots as i32);
    out_packet.write_int(0);

    out_packet
}

pub async fn on_check_duplicated_id_result(name: &str, result: CharacterIDResult) -> OutPacket {
    let mut out_packet = OutPacket::new(OutHeader::CheckDuplicatedIdResult);
    out_packet.write_string(name.to_string());
    out_packet.write_byte(result as u8);

    out_packet
}