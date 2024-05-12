use std::fmt::{Display, Formatter};
use std::sync::{Arc};
use std::time::Duration;
use log::{debug, error, info};
use tokio::io::{AsyncWriteExt, WriteHalf};
use tokio::net::TcpStream;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::{Mutex, RwLock};
use tokio::time;
use tokio::time::Instant;
use fungus_packet_utils::crypto::packet_coder::PacketCoder;
use uuid::Uuid;
use fungus_database::models::user::User;
use fungus_packet_utils::in_packet::InPacket;
use fungus_packet_utils::out_packet::OutPacket;
use fungus_packet_utils::packet_errors::PacketError;
use fungus_utils::constants::server_constants::{DEFAULT_RIV, DEFAULT_SIV};
use fungus_world::channel::Channel;
use crate::channels::client_channel::ClientChannel;
use crate::packets::login_packets::login_packets::on_send_connect;
use crate::packets::operation_handler::handle_packet;

pub struct ClientSession {
    session_id: Uuid,
    pub ip: String,
    pub machine_id: String,
    client_channel: Arc<RwLock<ClientChannel>>,
    pub sender: Sender<Vec<u8>>,
    pub packet_encoder: Mutex<PacketCoder>,

    // Game Stuff
    pub channel: Option<Channel>,
    pub user: Option<User>,
}

impl Display for ClientSession {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[ClientSession] ID: {}; IP; {}", self.session_id, self.ip)
    }
}

impl ClientSession {
    pub fn new(ip: String, client_channel: Arc<RwLock<ClientChannel>>, sender: Sender<Vec<u8>>) -> Self {
        ClientSession {
            session_id: Uuid::new_v4(),
            ip,
            machine_id: "".to_string(),
            client_channel,
            user: None,
            sender,
            packet_encoder: Mutex::from(PacketCoder::default())
        }
    }

    pub async fn send_handshake(&mut self) -> Result<(), PacketError> {
        info!("{}", self);
        if self.sender.send(
            on_send_connect(
                &DEFAULT_SIV,
                &DEFAULT_RIV
            ).as_bytes()
        ).await.is_err() {
            error!("Channel send error, likely receiver has dropped.");
            return Err(PacketError::CommunicationError());
        }
        Ok(())
    }

    pub async fn send_packet(&mut self, out_packet: &OutPacket) -> Result<(), PacketError> {
        let encoded_packet = {
            self.packet_encoder.lock().await.encode(&out_packet)
        };
        if self.sender.send(encoded_packet).await.is_err() {
            error!("Channel send error, likely receiver has dropped.");
            return Err(PacketError::CommunicationError());
        }
        Ok(())
    }

    pub fn get_user_ref(&self) -> Option<&User> {
        match &self.user {
            None => {
                None
            }
            Some(user_ref) => {
                Option::from(user_ref)
            }
        }
    }

    pub async fn get_siv(&self) -> [u8; 16] {
        self.packet_encoder.lock().await.get_siv()
    }

    pub async fn get_riv(&self) -> [u8; 16] {
        self.client_channel.read().await.packet_decoder.lock().await.get_riv()
    }

}
