use std::collections::HashMap;
use std::sync::{Arc};
use log::{error, info};
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{mpsc, Mutex, RwLock};
use tokio::task;
use fungus_net::channels::client_channel;
use fungus_net::channels::client_channel::ClientChannel;
use fungus_packet_utils::in_packet::InPacket;
use fungus_packet_utils::out_packet::OutPacket;
use fungus_packets::login_packets::login_packets::on_send_connect;
use fungus_packets::session::client_session::ClientSession;
use fungus_utils::constants::server_constants::{DEFAULT_RIV, DEFAULT_SIV, LOGIN_PORT, MAX_PACKET_SIZE, SERVER_IP};

pub struct LoginServer {
    channels: Arc<Mutex<HashMap<String, Arc<RwLock<ClientChannel>>>>>,
    sessions: Arc<Mutex<HashMap<String, ClientSession>>>
}

impl LoginServer {
    pub fn new() -> Self{
        LoginServer {
            channels: Arc::new(Mutex::new(HashMap::<String,Arc<RwLock<ClientChannel>>>::new())),
            sessions: Arc::new(Mutex::new(HashMap::<String,ClientSession>::new())),
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

                            let (tx, rx) = mpsc::channel::<[u8; MAX_PACKET_SIZE]>(16);
                            //let (send_out_packet, recv_out_packet) = mpsc::channel(16);
                            //let (send_in_packet, recv_in_packet) = mpsc::channel(16);

                            /*
                            let client_channel = Arc::new(RwLock::new(ClientChannel::new(
                                send_in_packet,
                                recv_out_packet,
                            )));

                            let client_session = Arc::new(RwLock::new(ClientSession::new(
                                addr.to_string(),
                                "n/a".to_string(),
                                client_channel.clone(),
                                recv_in_packet,
                                send_out_packet
                            )));

                            info!("New connection to login_server IP {}; SESS_ID {};", addr.to_string(), "asd".to_string());
                            // Uncomment and adapt if you have logic for handling client sessions
                            // self.sessions.lock().await.insert(
                            //     client_session.read().await.get_session_id(),
                            //     client_session.clone()
                            // );

                            self.channels.lock().await.insert(
                                "1".to_string(),
                                client_channel.clone()
                            );
                            task::spawn(ClientChannel::channel_handler(client_channel.clone(), on_send_connect(
                                &DEFAULT_SIV,
                                &DEFAULT_RIV
                            ), socket));
                            task::spawn(ClientSession::session_handler(client_session.clone()));
                             */
                        },
                        Err(e) => {
                            error!("Failed to establish a connection: {}", e);
                        }
                    }
                }
            },
            Err(e) => {
                error!("Failed to bind to {}: {}", &address, e);
            }
        }
        info!("Login Server closed the connection");
    }

}