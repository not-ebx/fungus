pub mod packets;
pub mod handlers;
pub mod enums;

use std::sync::Arc;
use std::time::Duration;
use env_logger::Builder;
use log::{error, info, LevelFilter};
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::sync::Mutex;
use tokio::time;
use fungus_client::session::client_session::ClientSession;
use fungus_packets::in_packet::InPacket;
use crate::handlers::operation_handler::handle_packet;
use crate::packets::login_packets::{on_send_connect, on_send_ping};

async fn handle_login_session(mut session_instance: Arc<Mutex<ClientSession>>) {
    loop {
        let mut session = session_instance.lock().await;
        // Read data from the socket
        match session.read().await {
            Ok(None) => {
                info!("Connection closed.");
                break;
            }
            Ok(Some(in_packet)) => {
                // Process received data if enough bytes are read
                handle_packet(in_packet);
            }
            Err(e) => {
                println!("Failed to read from socket: {}", e);
                break;
            }
        }
    }
}

async fn ping_session(session: Arc<Mutex<ClientSession>>) {
    let mut interval = time::interval(Duration::from_secs(1));
    loop {
        interval.tick().await;
        let mut cur_session = session.lock().await;
        if let Err(e) = cur_session.write(&on_send_ping()).await {
            error!("Failed to send ping to session {}", cur_session.get_session_id());
            break;
        }
    }
}

#[tokio::main]
async fn main() {
    Builder::new()
        .filter(None, LevelFilter::Info)
        .init();
    let listener = TcpListener::bind("127.0.0.1:8484").await.unwrap();

    while let Ok((tcp_stream, _)) = listener.accept().await {
        let socket = Arc::new(Mutex::new(tcp_stream));
        let session = Arc::new(Mutex::new(ClientSession::new(
            "127.0.0.1".to_string(),
            "sexo".to_string(),
            socket,
            [70,114,122,82],
            [82,48,120,115]
        )));

        let mut session_mutex = session.lock().await;
        &session_mutex.write_handshake(
            &on_send_connect(
                &[70,114,122,82],
                &[82,48,120,115]
            )
        ).await.expect("TODO: panic message");

        tokio::spawn(ping_session(session.clone()));
        tokio::spawn(handle_login_session(session.clone()));
    }
}