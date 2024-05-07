use fungus_database::models::user::User;
use fungus_database::schema::users::{account_type, password};
use fungus_utils::constants::server_constants::{ALLOW_AUTO_REGISTER, LOCALE, MINOR_VERSION, VERSION};
use fungus_utils::enums::login_type::LoginType;
use fungus_packet_utils::out_headers::OutHeader;
use fungus_packet_utils::out_packet::OutPacket;
use fungus_utils::types::fungus_time::FungusTime;
use rand::{Rng, RngCore};

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

pub fn on_check_password_result(user: Option<User>, success_code: LoginType, ) -> OutPacket {
    match success_code {
        LoginType::Success => {
            let found_user = user.unwrap();
            on_success_login(found_user)
        }
        LoginType::NotRegistered => {
            // Send not registered packet
            // TODO fix this lmao
            on_send_ping()
        }

        LoginType::IncorrectPassword => {
            // Send not registered packet
            // TODO fix this lmao
            on_send_ping()
        }
        /*
        LoginType::TempBlocked => {}
        LoginType::Blocked => {}
        LoginType::Abandoned => {}

        LoginType::DBFail => {}
        LoginType::AlreadyConnected => {}
        LoginType::NotConnectableWorld => {}
         */
        _ => {OutPacket::default()}
    }
}

// This has to own the user.
pub fn on_success_login(user: User) -> OutPacket {
    let mut out_packet: OutPacket= OutPacket::new(OutHeader::CheckPasswordResult);

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
    out_packet.write_string(user.username);
    out_packet.write_byte(3); // 3 for new accds .. ?
    out_packet.write_byte(0); // quiet ban
    out_packet.write_long(0); // quiet ban time
    out_packet.write_byte(1); // idk?

    // Get the time
    let ms_time = FungusTime::from(user.created_at.clone());

    out_packet.write(
        ms_time
    );

    // Something to select the world
    out_packet.write_int(4);
    out_packet.write_byte(1); // Pin Disabled
    out_packet.write_byte(2); // Pic Disabled, 2

    let mut rng = rand::thread_rng();
    out_packet.write_long(rng.next_u64() as i64); // TODO gotta create a randomizer :)

    out_packet

}