use crate::session::client_session::ClientSession;
use fungus_packet_utils::in_headers::InHeader;
use fungus_packet_utils::in_packet::InPacket;
use fungus_packet_utils::out_packet::OutPacket;
use fungus_packet_utils::packet_errors::PacketError;
use fungus_packet_utils::types::packet_buffer::PacketBuffer;
use fungus_utils::constants::server_constants::{DEFAULT_RIV, DEFAULT_SIV};
use log::{info, warn};
use std::ops::DerefMut;
use std::sync::{Arc, RwLock};
use tokio::sync::MutexGuard;
use crate::packets::login_packets::handlers::login_handlers::handle_check_login_auth_info;
use crate::packets::login_packets::login_packets::on_send_connect;

//pub async fn handle_packet(session: &mut ClientSession, mut packet: InPacket) {
pub async fn handle_packet(session: &mut ClientSession, mut packet: InPacket) -> Option<OutPacket> {
    let opcode = packet.get_header();
    let packet = match opcode {
        //InHeader::BeginSocket => {}
        InHeader::CheckLoginAuthInfo => handle_check_login_auth_info(session, &mut packet).await,
        /*
        InHeader::GuestLogin => {}
        InHeader::SelectPreviousWorld => {}
        InHeader::SelectWorld => {}
        InHeader::WorldStatusRequest => {}
        InHeader::EULA => {}
        InHeader::ClientStart=> {}
        InHeader::WorldInfoRequest => {}
         */
        InHeader::VersionVerify => Ok(on_send_connect(&DEFAULT_SIV, &DEFAULT_RIV)),
        //InHeader::UNKNOWN => {}
        _ => Err(PacketError::UnimplementedPacket(format!("{}", packet))),
    };

    match packet {
        Ok(out_packet) => Some(out_packet),
        Err(e) => {
            warn!("{}", e);
            None
        }
    }
}
