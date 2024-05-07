use env_logger::Builder;
use log::LevelFilter;
use tokio::task;
use fungus_login::acceptor::LoginServer;

#[tokio::main]
async fn main() {
    Builder::new()
        .filter(None, LevelFilter::Info)
        .init();
    let mut login_server= LoginServer::new();

    let login_handler = tokio::spawn(async move {
        login_server.listen().await
    }).await;
}
