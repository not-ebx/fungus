pub mod packets;
pub mod handlers;
pub mod enums;

use env_logger::Builder;
use log::{info, LevelFilter};
use tokio::net::TcpListener;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use fungus_client::session::client_session::ClientSession;
use fungus_packets::in_packet::InPacket;
use crate::packets::login_packets::{on_send_connect, on_send_ping};

#[tokio::main]
async fn main() -> io::Result<()> {
    Builder::new()
        .filter(None, LevelFilter::Info)
        .init();
    let listener = TcpListener::bind("192.168.1.85:8484").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        let mut session_test = ClientSession::new(
            "127.0.0.1".to_string(),
            "sexo".to_string(),
            socket,
            [70,114,122,82],
            [82,48,120,115]
        );

        tokio::spawn(async move {
            let mut buf = [0u8; 1024];
            session_test.write_handshake(
                &on_send_connect(
                    &session_test.get_siv()[0..4],
                    &session_test.get_riv()[0..4]
                )
            ).await.expect("TODO: panic message");

            session_test.write(
                &on_send_ping()
            ).await.expect("TODO: panic message");

            // Read data from the socket
            match session_test.read().await {
                Ok(None) => {
                    info!("Connection closed.");
                }
                Ok(Some(n)) => {
                    // Process received data if enough bytes are read
                    info!("TODO: Handle the packet.")
                }
                Err(e) => {
                    println!("Failed to read from socket: {}", e);
                }
            }
        });
    }
}