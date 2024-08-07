use std::sync::Arc;
use std::thread;
use env_logger::Builder;
use fungus_login::acceptor::LoginServer;
use log::LevelFilter;
use tokio::runtime::Runtime;
use fungus_game::services::service_registry::ServiceRegistry;
use fungus_net::server::server::Server;
use fungus_utils::fg_printc_info;

#[tokio::main]
async fn main() {
    Builder::new().filter(None, LevelFilter::Info).init();

    fg_printc_info!("Starting Fungus v0.0.1 Alpha. What the sigma");
    let service_registry = Arc::from(ServiceRegistry::new());
    let server_instance = Arc::from(Server::new(service_registry.clone()));

    let login_service_registry = service_registry.clone();
    let login_server = || async {
        let mut login_server = LoginServer::new(
            server_instance,
            login_service_registry
        );
        login_server.listen().await;
    };

    let login_handler = thread::spawn(move || {
        let rt = Runtime::new().unwrap();
        rt.block_on(login_server());
    });

    login_handler.join().unwrap();
}
