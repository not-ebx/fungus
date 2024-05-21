use std::sync::Arc;
use std::time::Duration;

use log::{error, warn};
use tokio::sync::mpsc::{Receiver};
use tokio::sync::Mutex;
use tokio::time;
use tokio::time::Instant;

use fungus_packet_utils::crypto::packet_coder::PacketCoder;
use fungus_packet_utils::packet_errors::PacketError;

use crate::packets::login_packets::login_packets::on_send_ping;
use crate::packets::operation_handler::handle_packet;
use crate::session::client_session::ClientSession;

pub struct ClientChannel {
    pub receiver: Receiver<Vec<u8>>,
    pub packet_decoder: Mutex<PacketCoder>,
}

impl ClientChannel {
    pub fn new(receiver: Receiver<Vec<u8>>) -> Self {
        ClientChannel {
            receiver,
            packet_decoder: Mutex::new(Default::default()),
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
                    let opcode = in_packet.get_header().clone();
                    let mut session_guard = client_session.lock().await;
                    match handle_packet(&mut session_guard, in_packet).await {
                        Ok(_) => {}
                        Err(e) => {
                            match e {
                                PacketError::UnimplementedPacket(_) |
                                PacketError::UnhandledHeader(_) |
                                PacketError::UnknownHeader(_) => {
                                    if !opcode.is_ignored() {
                                        warn!("{}", e);
                                    }
                                },
                                _ => {
                                    error!("{}", e);
                                }
                            }
                        }
                    }
                },
                _ = interval.tick() => {
                    let mut session_guard = client_session.lock().await;
                    if let Err(e) = session_guard.send_packet(&on_send_ping()).await {
                        error!("Failed to send ping: {}", e);
                    }
                }
            }
        }
    }

}
