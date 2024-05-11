use std::sync::Arc;
use std::time::Duration;
use log::{error, info};
use fungus_packet_utils::crypto::packet_coder::PacketCoder;
use fungus_packet_utils::in_packet::InPacket;
use fungus_packet_utils::out_packet::OutPacket;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::Mutex;
use tokio::time;
use tokio::time::Instant;
use fungus_packet_utils::packet_errors::PacketError;
use fungus_utils::constants::server_constants::{DEFAULT_RIV, DEFAULT_SIV};
use crate::packets::login_packets::login_packets::{on_send_connect, on_send_ping};
use crate::packets::operation_handler::handle_packet;
use crate::session::client_session::ClientSession;

pub struct ClientChannel {
    pub receiver: Receiver<Vec<u8>>,
    //pub sender: Sender<OutPacket>,
    pub packet_decoder: Mutex<PacketCoder>,
    //pub packet_encoder: Mutex<PacketCoder>,
}

impl ClientChannel {
    pub fn new(receiver: Receiver<Vec<u8>>) -> Self {
        ClientChannel {
            receiver,
            //sender,
            packet_decoder: Mutex::new(Default::default()),
            //packet_encoder: Mutex::new(Default::default()),
        }
    }

    pub async fn handle_inbound(&mut self, mut client_session: Arc<Mutex<ClientSession>>) {
        {
            let handshake_result = client_session.lock().await.send_handshake().await;
            match handshake_result {
                Ok(_) => {}
                Err(e) => {
                    error!("{}", e);
                    return;
                }
            }
        }

        let mut interval = time::interval_at(Instant::now() + Duration::from_secs(10), Duration::from_secs(10));

        loop {
            tokio::select! {
                Some(packet) = self.receiver.recv() => {
                    let in_packet = {
                        let mut decoder = self.packet_decoder.lock().await;
                        decoder.decode(&packet)
                    };
                    let mut session_guard = client_session.lock().await;
                    match handle_packet(&mut session_guard, in_packet).await {
                        Ok(_) => {
                            info!("Successfully handled packet");
                        }
                        Err(e) => {
                            error!("{}", e);
                        }
                    }
                },
                _ = interval.tick() => {
                    let mut session_guard = client_session.lock().await;
                    match session_guard.send_packet(&on_send_ping()).await {
                        Ok(_) => info!("Ping sent successfully."),
                        Err(e) => {
                            error!("Failed to send ping: {}", e);
                            break;
                        }
                    }
                }
            }
        }
    }

    /*
    pub async fn handle_inbound(&mut self, mut client_session: Arc<Mutex<ClientSession>>) {
        {
            let session_guard = client_session.lock().await;
            info!("{}", session_guard);
            self.sender.send(
                on_send_connect(
                    &DEFAULT_SIV,
                    &DEFAULT_RIV
                )
            ).await.expect("Nones");
            info!("Sent handshake to client, creating ping task too.");
        }

        while let Some(packet) = self.receiver.recv().await {
            let mut decoder = self.packet_decoder.lock().await;
            // Assume `decode_packet` is an async function in `PacketCoder`
            let in_packet = decoder.decode(&packet);
            // TODO handle
            let mut session_guard = client_session.lock().await;
            match handle_packet(&mut session_guard, in_packet).await {
                Some(out_packet) => {
                    // Encode the packet before sending it
                    let encoded_packet = self.packet_encoder.lock().await.encode(&out_packet);
                    if self.sender.send(encoded_packet).await.is_err() {
                        error!("Channel send error, likely receiver has dropped.");
                        break;
                    }
                }
                None => {}
            }
        }
    }

    pub async fn send_alive_req(&mut self) {
        let delay = Duration::from_millis(10000);
        loop {
            time::sleep(delay).await;
            // encode packet and shit
            let encoded_packet = self.packet_encoder.lock().await.encode(&on_send_ping());
            if let Err(e) = self.sender.send(encoded_packet).await {
                println!("Failed to send ping: {}", e);
                break;
            }


        }
    }
    */
    pub fn handle_outbound(&mut self, mut client_session: Arc<Mutex<ClientSession>>) {

    }
}
