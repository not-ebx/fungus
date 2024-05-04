use std::sync::Arc;
use fungus_packets::crypto::packet_coder::PacketCoder;
use fungus_packets::out_packet::OutPacket;
use uuid::{Uuid};
use fungus_packets::in_packet::InPacket;
use log::info;
use tokio::io;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::Mutex;

pub struct ClientSession {
    session_id: Uuid,
    ip: String,
    machine_id: String,
    socket: Arc<Mutex<TcpStream>>,
    packet_coder: PacketCoder
}

impl ClientSession {

    pub fn new(ip: String, machine_id: String, socket: Arc<Mutex<TcpStream>>, siv: [u8; 4], riv: [u8;4]) -> Self {
        ClientSession {
            session_id: Uuid::new_v4(),
            ip,
            machine_id,
            socket,
            packet_coder: PacketCoder::new(siv, riv)
        }
    }

    pub fn get_session_id(&self) -> String{
        self.session_id.to_string()
    }

    pub async fn read(&mut self) -> io::Result<Option<InPacket>> {
        let mut buf = [0u8; 65535];  // Adjust size according to expected data volume
        let mut socket = self.socket.lock().await;
        let data_size = socket.read(&mut buf).await?;

        if data_size == 0 {
            return Ok(None);
        }

        let in_packet = self.packet_coder.decode(&buf[..data_size]);

        Ok(Some(in_packet))
    }

    pub async fn write(&mut self, out_packet: &OutPacket) -> std::io::Result<()> {
        let mut socket = self.socket.lock().await;
        let encoded_packet = self.packet_coder.encode(
            out_packet
        );

        socket.write_all(
            &*encoded_packet
        ).await?;

        socket.flush().await
    }

    pub async fn write_handshake(&mut self, handshake: &OutPacket) -> std::io::Result<()> {
        let mut socket = self.socket.lock().await;
        info!("Sending Handshake\n{}", handshake);
        socket.write_all(
            &handshake.get_bytes()
        ).await?;

        socket.flush().await
    }

    pub fn get_siv(&self) -> [u8; 16] {
        self.packet_coder.get_siv()
    }

    pub fn get_riv(&self) -> [u8; 16] {
        self.packet_coder.get_riv()
    }
}