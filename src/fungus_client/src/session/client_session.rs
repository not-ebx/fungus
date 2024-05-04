use std::io::{Read, Write};
use tokio::io;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use fungus_packets::crypto::packet_coder::PacketCoder;
use fungus_packets::out_packet::OutPacket;
use uuid::{Uuid};
use fungus_packets::in_packet::InPacket;
use log::info;

pub struct ClientSession {
    session_id: Uuid,
    ip: String,
    machine_id: String,
    socket: TcpStream,
    packet_coder: PacketCoder
}

impl ClientSession {
    pub fn new(ip: String, machine_id: String, socket: TcpStream, siv: [u8; 4], riv: [u8;4]) -> Self {
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
        let data_size = self.socket.read(&mut buf).await?;

        if data_size == 0 {
            // EOF reached, meaning the connection has been closed
            return Ok(None);
        }

        // Now we assume the packet_coder.decode() can handle partial data or requires full packets.
        // This method should handle or report an error if the data is not a complete packet
        let in_packet = self.packet_coder.decode(&buf[..data_size]);

        // You need to define how to handle decoding errors if `decode()` can fail
        /*match in_packet {
            Ok(packet) => Ok(Some(packet)),
            Err(e) => Err(io::Error::new(io::ErrorKind::InvalidData, e)),
        }*/
        Ok(Some(in_packet))
    }

    pub async fn write(&mut self, out_packet: &OutPacket) -> std::io::Result<()> {
        let encoded_packet = self.packet_coder.encode(
            out_packet
        );
        self.socket.write_all(
            &*encoded_packet
        ).await
    }

    pub async fn write_handshake(&mut self, handshake: &OutPacket) -> std::io::Result<()> {
        info!("Sending Handshake\n{}", handshake);
        self.socket.write_all(
            &handshake.get_bytes()
        ).await
    }

    pub fn get_siv(&self) -> [u8; 16] {
        self.packet_coder.get_siv()
    }

    pub fn get_riv(&self) -> [u8; 16] {
        self.packet_coder.get_riv()
    }
}