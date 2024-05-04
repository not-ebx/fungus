use log::info;
use fungus_packets::in_headers::InHeader;
use fungus_packets::in_packet::InPacket;
use crate::handlers::login_handlers::handle_check_login_auth_info;

pub fn handle_packet(mut packet: InPacket) {
    let opcode = packet.get_header();
    match opcode {
        //InHeader::BeginSocket => {}
        InHeader::CheckLoginAuthInfo => {
            handle_check_login_auth_info(&mut packet).expect("TODO: panic message");
        }
        InHeader::GuestLogin => {}
        InHeader::SelectPreviousWorld => {}
        InHeader::SelectWorld => {}
        InHeader::WorldStatusRequest => {}
        InHeader::EULA => {}
        InHeader::CLIENT_START => {}
        InHeader::WorldInfoRequest => {}
        InHeader::UNKNOWN => {}

        _ => {
            info!("Unhandled packet: {}", packet);
        }
    }
}