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
use crate::packets::client_packets::client_packets::handle_client_error;
use crate::packets::login_packets::handlers::login_handlers::{handle_check_duplicate_id, handle_check_login_auth_info, handle_create_new_character, handle_select_world, handle_world_list_request, handle_world_status_request};
use crate::packets::login_packets::login_packets::on_send_connect;

//pub async fn handle_packet(session: &mut ClientSession, mut packet: InPacket) {
pub async fn handle_packet(session: &mut ClientSession, mut packet: InPacket) -> Result<(), PacketError> {
    let opcode = packet.get_header();
    match opcode {
        //InHeader::BeginSocket => {}
        InHeader::CheckLoginAuthInfo => {
            handle_check_login_auth_info(session, &mut packet).await
        }
        InHeader::WorldListRequest | InHeader::WorldInfoRequest | InHeader::RedisplayWorldList => {
            handle_world_list_request(session, packet).await
        }
        InHeader::WorldStatusRequest => {
            handle_world_status_request(session, &mut packet).await
        }
        InHeader::SelectWorld => {
            handle_select_world(session, &mut packet).await
        }
        InHeader::CheckDuplicateID => {
            handle_check_duplicate_id(session, &mut packet).await
        }
        InHeader::CreateNewCharacter => {
            handle_create_new_character(session, &mut packet).await
        }
        /*
        InHeader::GuestLogin => {}
        InHeader::SelectPreviousWorld => {}
        InHeader::SelectWorld => {}
        InHeader::WorldStatusRequest => {}
        InHeader::EULA => {}
        InHeader::ClientStart=> {}
        InHeader::WorldInfoRequest => {}
         */
        InHeader::VersionVerify => {
            session.send_packet(
                &on_send_connect(&DEFAULT_SIV, &DEFAULT_RIV)
            ).await
        },
        //InHeader::UNKNOWN => {}
        InHeader::ClientError => {
            handle_client_error(session, &mut packet).await
        }
        _ => Err(PacketError::UnimplementedPacket(format!("{}", packet))),
    }
}
