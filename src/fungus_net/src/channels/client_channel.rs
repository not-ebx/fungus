use std::sync::Arc;
use log::{error, info};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::{Mutex, RwLock};
use tokio::sync::mpsc::error::SendError;
use fungus_packet_utils::crypto::packet_coder::PacketCoder;
use fungus_packet_utils::in_packet::InPacket;
use fungus_packet_utils::out_packet::OutPacket;
use fungus_utils::constants::server_constants::MAX_PACKET_SIZE;

pub struct ClientChannel {
    sender: Sender<InPacket>,
    receiver: Receiver<OutPacket>,
    packet_decoder: Mutex<PacketCoder>,
    packet_encoder: Mutex<PacketCoder>,
}

impl ClientChannel {
    pub fn new(sender: Sender<InPacket>, receiver: Receiver<OutPacket>) -> Self {
        ClientChannel {
            sender,
            receiver,
            packet_decoder: Mutex::from(PacketCoder::default()),
            packet_encoder: Mutex::from(PacketCoder::default()),
        }
    }

    pub async fn get_riv(&self) -> [u8; 16] {
        self.packet_decoder.lock().await.get_riv()
    }

    pub async fn get_siv(&self) -> [u8; 16] {
        self.packet_decoder.lock().await.get_siv()
    }

    pub async fn channel_handler(client_channel: Arc<RwLock<ClientChannel>>, handshake_packet: OutPacket, mut socket: TcpStream) {
        let mut buf = [0u8; MAX_PACKET_SIZE];
        // Create the handshake
        info!("Sent handshake~");
        socket.write_all(
            &*handshake_packet.as_bytes()
        ).await.expect("Could NOT send handshake.");
        loop {
            let buf_size = {
                socket.read(&mut buf).await
            };
            match buf_size {
                Ok(0) => {
                    // Dead?
                    info!("Idk if deead or what lol");
                    break;
                }
                Ok(size) => {
                    client_channel.write().await.send_in_packet(buf).await;
                }
                Err(_) => {}
            }

            let some_packet = {
                client_channel.write().await.receiver.recv().await
            };
            match some_packet {
                None => {
                    error!("No packet..?");
                    break;
                }
                Some(packet) => {
                    let encoded_packet = {
                        client_channel.write().await.encode_out(&packet)
                    }.await;
                    socket.write_all(
                        &*encoded_packet.as_bytes()
                    ).await.expect("TODO: panic message");
                }
            }
        }
    }

    pub async fn encode_out(&mut self, out_packet: &OutPacket) -> OutPacket {
        let mut decoder = self.packet_encoder.lock().await;
        decoder.encode(out_packet)
    }

    pub async fn send_in_packet(&mut self, buf: [u8; 65535]) {
        if buf.len() == 0 {
            return;
        }

        let in_packet = {
            let mut decoder = self.packet_decoder.lock().await;
            decoder.decode(&buf[..buf.len()])
        };


        match self.sender.send(in_packet).await {
            Ok(_) => {}
            Err(e) => { error!("{}", e)}
        }

    }
}