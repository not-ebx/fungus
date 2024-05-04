use fungus_database::models::user::User;
use fungus_packets::out_headers::OutHeader;
use fungus_packets::out_packet::OutPacket;
use fungus_utils::constants::server_constants::{LOCALE, MINOR_VERSION, VERSION};
use crate::enums::login_type::LoginType;

pub fn on_send_ping() -> OutPacket {
    OutPacket::new(OutHeader::AliveReq)
}

pub fn on_send_connect(siv: &[u8], riv: &[u8]) -> OutPacket {
    // Length = 13 + "1"
    let mut out_packet: OutPacket = OutPacket::default();

    out_packet.write_short(14); // Size of packet
    out_packet.write_short(VERSION);
    out_packet.write_string(MINOR_VERSION);
    out_packet.write_bytes(&siv.to_vec());
    out_packet.write_bytes(&riv.to_vec());
    out_packet.write_byte(LOCALE);

    out_packet
}

pub fn on_check_password_result(user: Option<User>, success_code: LoginType, ) {
    match success_code {
        LoginType::Success => {
            let found_user = user.unwrap();
            on_success_login(found_user);
        }
        LoginType::TempBlocked => {}
        LoginType::Blocked => {}
        LoginType::Abandoned => {}
        LoginType::IncorrectPassword => {}
        LoginType::NotRegistered => {}
        LoginType::DBFail => {}
        LoginType::AlreadyConnected => {}
        LoginType::NotConnectableWorld => {}
        _ => {}
    }
}

// This has to own the user.
pub fn on_success_login(user: User) {
    let mut out_packet: OutPacket= OutPacket::new(OutHeader::CheckPasswordResult);

    // A byte idk
    // Success code
    out_packet.write_byte(LoginType::Success as u8);


}