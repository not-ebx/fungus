use fungus_net::channels::client_channel;
use fungus_net::channels::client_channel::ClientChannel;
use fungus_packet_utils::in_packet::InPacket;
use fungus_packet_utils::out_packet::OutPacket;
use fungus_packet_utils::types::packet_buffer::PacketBuffer;
use fungus_utils::constants::server_constants::{
    DEFAULT_RIV, DEFAULT_SIV, LOGIN_PORT, MAX_PACKET_SIZE, SERVER_IP,
};
use log::{debug, error, info};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::{ReadHalf, WriteHalf};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{mpsc, Mutex, RwLock};
use tokio::sync::mpsc::Sender;
use tokio::{task, time};
use tokio::time::Instant;
use fungus_net::packets::login_packets::login_packets::{on_send_connect};
use fungus_net::packets::operation_handler::handle_packet;
use fungus_net::session::client_session::ClientSession;

pub struct LoginServer {
    channels: Arc<Mutex<HashMap<String, Arc<RwLock<ClientChannel>>>>>,
    sessions: Arc<Mutex<HashMap<String, ClientSession>>>,
}

async fn read_packets(mut socket: tokio::io::ReadHalf<TcpStream>, tx: mpsc::Sender<Vec<u8>>) {
    let mut buf = [0u8; MAX_PACKET_SIZE];
    loop {
        match socket.read(&mut buf).await {
            Ok(0) => {
                error!("Size 0 buffer, ded maybe lol owned :dab:");
                break;
            }
            Ok(_) => {
                if tx.send(buf.to_vec()).await.is_err() {
                    error!("Channel send error, likely receiver has dropped.");
                    break;
                }
            }
            Err(e) => {
                error!("Failed to read from socket: {}", e);
                break;
            }
        }
    }
}

async fn write_packets(mut socket: tokio::io::WriteHalf<TcpStream>, mut rx: mpsc::Receiver<Vec<u8>>) {
    loop {
        match rx.recv().await {
            None => {}
            Some(packet) => {
                if let Err(e) = socket.write_all(
                    packet.as_slice()
                ).await {
                    error!("An error occurred trying to write the Outbound Packet: {}", e);
                    break;
                }
            }
        }
    }
}

impl LoginServer {
    pub fn new() -> Self {
        LoginServer {
            channels: Arc::new(Mutex::new(
                HashMap::<String, Arc<RwLock<ClientChannel>>>::new(),
            )),
            sessions: Arc::new(Mutex::new(HashMap::<String, ClientSession>::new())),
        }
    }
    pub async fn listen(&mut self) {
        let address = format!("{}:{}", &SERVER_IP, &LOGIN_PORT);
        let listener = TcpListener::bind(&address).await;

        match listener {
            Ok(listener) => {
                info!("Login server is now listening @ {}", &address);

                loop {
                    match listener.accept().await {
                        Ok((mut socket, addr)) => {
                            if let Err(e) = socket.set_nodelay(true) {
                                error!("Failed to set TCP_NODELAY on socket: {}", e);
                                continue;
                            }

                            let (socket_read, socket_write) = tokio::io::split(socket);

                            // Outbound Channel
                            let (send_out_packet, recv_out_packet) = mpsc::channel::<Vec<u8>>(32);
                            // Inbound Channel
                            let (send_in_packet, recv_in_packet) = mpsc::channel::<Vec<u8>>(32);

                            let client_channel = Arc::new(RwLock::new(
                                ClientChannel::new(recv_in_packet)
                            ));

                            let client_session =
                                Arc::new(Mutex::new(ClientSession::new(
                                    addr.to_string(),
                                    client_channel.clone(),
                                    send_out_packet
                                )));

                            info!(
                                "New connection to login_server IP {}; SESS_ID {};",
                                addr.to_string(),
                                "asd".to_string()
                            );
                            // Write handshake

                            tokio::spawn(async move {
                                read_packets(socket_read, send_in_packet).await;
                            });

                            tokio::spawn(async move {
                                write_packets(socket_write, recv_out_packet).await;
                            });

                            tokio::spawn(async move {
                                client_channel.clone().write().await.handle_inbound(
                                    client_session.clone()
                                ).await
                            });

                            /*
                            // Uncomment and adapt if you have logic for handling client sessions
                            // self.sessions.lock().await.insert(
                            //     client_session.read().await.get_session_id(),
                            //     client_session.clone()
                            // );

                            self.channels.lock().await.insert(
                                "1".to_string(),
                                client_channel.clone()
                            );

                            task::spawn(ClientSession::session_handler(client_session.clone()));
                             */
                        }
                        Err(e) => {
                            error!("Failed to establish a connection: {}", e);
                        }
                    }
                }
            }
            Err(e) => {
                error!("Failed to bind to {}: {}", &address, e);
            }
        }
        info!("Login Server closed the connection");
    }
}
