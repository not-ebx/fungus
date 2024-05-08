use std::fmt::{Display, Formatter};
use std::sync::Arc;
use log::{debug, error, info};
use tokio::io::{AsyncWriteExt, WriteHalf};
use tokio::net::TcpStream;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::Mutex;
use fungus_packet_utils::crypto::packet_coder::PacketCoder;
use uuid::Uuid;
use fungus_packet_utils::in_packet::InPacket;
use fungus_packet_utils::out_packet::OutPacket;
use fungus_utils::constants::server_constants::{DEFAULT_RIV, DEFAULT_SIV};
use crate::channels::client_channel::ClientChannel;
use crate::packets::login_packets::login_packets::on_send_connect;
use crate::packets::operation_handler::handle_packet;

pub struct ClientSession {
    session_id: Uuid,
    pub ip: String,
    machine_id: String,
    pub client_channel: Arc<Mutex<ClientChannel>>
}

impl Display for ClientSession {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[ClientSession] ID: {}; IP; {}", self.session_id, self.ip)
    }
}

impl ClientSession {
    pub fn new(ip: String, client_channel: Arc<Mutex<ClientChannel>>) -> Self {
        ClientSession {
            session_id: Uuid::new_v4(),
            ip,
            machine_id: "".to_string(),
            client_channel
        }
    }


    pub async fn get_siv(&self) -> [u8; 16] {
        self.client_channel.lock().await.packet_encoder.lock().await.get_siv()
    }

    pub async fn get_riv(&self) -> [u8; 16] {
        self.client_channel.lock().await.packet_decoder.lock().await.get_riv()
    }
}
