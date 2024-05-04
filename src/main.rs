use tokio::sync::mpsc;
use std::sync::Arc;
use tokio::sync::Mutex;
use fungus_client::session::client_session::ClientSession;

async fn login_server_thread(sender: mpsc::Sender<Arc<Mutex<ClientSession>>>) {
    // Example: Create a new session
    let session = Arc::new(Mutex::new(
        ClientSession::new(
            "127.0.0.1".to_string(),
            "sexo".to_string(),
            socket,
            [70,114,122,82],
            [82,48,120,115]
        )
    ));

    // Send the session to the world server
    sender.send(session).await.unwrap();
}

async fn world_server_thread(mut receiver: mpsc::Sender<Arc<Mutex<ClientSession>>>) {
    while let Some(session) = receiver.recv().await {
        let session = session.lock().await;
        println!("Received session: {:?}", session);
    }
}

#[tokio::main]
async fn main() {
    let (tx, rx) = mpsc::channel(100);

    let login_handle = tokio::spawn(async move {
        login_server_thread(tx).await;
    });

    let world_handle = tokio::spawn(async move {
        world_server_thread(rx).await;
    });

    login_handle.await.unwrap();
    world_handle.await.unwrap();
}
