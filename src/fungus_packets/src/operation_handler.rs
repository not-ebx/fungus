use std::ops::DerefMut;
use std::sync::{Arc, RwLock};
use log::info;
use fungus_packet_utils::in_headers::InHeader;
use fungus_packet_utils::in_packet::InPacket;
use crate::session::client_session::ClientSession;
use fungus_utils::constants::server_constants::{DEFAULT_RIV, DEFAULT_SIV};
use crate::login_packets::handlers::login_handlers::handle_check_login_auth_info;
use crate::login_packets::login_packets::on_send_connect;

//pub async fn handle_packet(session: &mut ClientSession, mut packet: InPacket) {
pub async fn handle_packet(session: &mut ClientSession, mut packet: InPacket) {
    let opcode = packet.get_header();
    match opcode {
        //InHeader::BeginSocket => {}
        InHeader::CheckLoginAuthInfo => {
            handle_check_login_auth_info(
                session, &mut packet
            ).await.expect("wow");
        }
        InHeader::GuestLogin => {}
        InHeader::SelectPreviousWorld => {}
        InHeader::SelectWorld => {}
        InHeader::WorldStatusRequest => {}
        InHeader::EULA => {}
        InHeader::ClientStart=> {}
        InHeader::WorldInfoRequest => {}
        InHeader::VersionVerify => {
            session.send_out_packet(on_send_connect(
                &DEFAULT_SIV,
                &DEFAULT_RIV
            )).await;
        }
        InHeader::UNKNOWN => {}

        _ => {
            info!("Unhandled packet: {}", packet);
        }
    }
}