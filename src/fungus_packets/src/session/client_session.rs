use std::ops::DerefMut;
use std::sync::Arc;
use log::info;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::RwLock;
use uuid::{Uuid};
use fungus_net::channels::client_channel::ClientChannel;
use fungus_packet_utils::in_packet::InPacket;
use fungus_packet_utils::out_packet::OutPacket;
use fungus_utils::constants::server_constants::MAX_PACKET_SIZE;
use crate::operation_handler::handle_packet;

pub struct ClientSession {
    session_id: Uuid,
    pub ip: String,
    machine_id: String,
    client_channel: Arc<RwLock<ClientChannel>>,
    pub recv_in: Receiver<InPacket>,
    pub send_out: Sender<OutPacket>
}

impl ClientSession {

    pub fn new(ip: String, machine_id: String, client_channel: Arc<RwLock<ClientChannel>>, recv_in: Receiver<InPacket>, send_out: Sender<OutPacket>) -> Self {
        ClientSession {
            session_id: Uuid::new_v4(),
            ip,
            machine_id,
            client_channel,
            recv_in,
            send_out
        }
    }

    pub async fn session_handler(client_session: Arc<RwLock<ClientSession>>) {
        let mut buf = [0u8; MAX_PACKET_SIZE];
        loop {
            let some_packet = {
                client_session.write().await.recv_in.recv().await
            };
            match some_packet {
                None => {
                    // Nothing happened lol
                }
                Some(packet) => {
                    handle_packet(
                        &mut *client_session.write().await,
                        packet
                    ).await;
                }
            }
        }
    }

    pub fn get_session_id(&self) -> String{
        self.session_id.to_string()
    }

    pub async fn listen_channel(&mut self) {
        let mut buf = [0u8; MAX_PACKET_SIZE];
        loop {
            match self.recv_in.recv().await {
                None => {}
                Some(packet) => {
                    handle_packet(self, packet).await;
                }
            }
        }
    }

    pub async fn send_out_packet(&self, out_packet: OutPacket) {
        info!("{}", out_packet);
        self.send_out.send(out_packet).await.expect("TODO: panic message");
    }

    /*
    pub async fn read(&mut self) -> io::Result<Option<InPacket>> {
        let mut buf = [0u8; 65535];  // Adjust size according to expected data volume
        let data_size = self.socket.read(&mut buf).await?;

        if data_size == 0 {
            return Ok(None);
        }

        let in_packet = self.packet_coder.decode(&buf[..data_size]);

        Ok(Some(in_packet))
    }

    pub async fn write(&mut self, out_packet: &OutPacket) -> Result<(), SendError<OutPacket>> {
        let encoded_packet = self.packet_coder.encode(
            out_packet
        );

        self.sender.send(encoded_packet).await
        /*
        self.socket.write_all(
            &*encoded_packet
        ).await?;

        self.socket.flush().await*/
    }

    pub async fn write_handshake(&mut self, handshake: &OutPacket) -> std::io::Result<()> {
        info!("Sending Handshake\n{}", handshake);
        self.socket.write_all(
            &handshake.get_bytes()
        ).await?;

        self.socket.flush().await
    }
     */

    pub async fn get_siv(&self) -> [u8; 16] {
        self.client_channel.clone().read().await.get_siv().await
    }

    pub async fn get_riv(&self) -> [u8; 16] {
        self.client_channel.clone().read().await.get_riv().await
    }
}