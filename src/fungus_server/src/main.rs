use std::sync::Arc;
use std::thread;
use env_logger::Builder;
use fungus_login::acceptor::LoginServer;
use log::LevelFilter;
use tokio::runtime::Runtime;
use fungus_game::services::service_registry::ServiceRegistry;
use fungus_net::server::server::Server;

#[tokio::main]
async fn main() {
    Builder::new().filter(None, LevelFilter::Info).init();

    let server_instance = Arc::from(Server::new());
    let service_registry = Arc::from(ServiceRegistry::new());

    let login_server = || async {
        let mut login_server = LoginServer::new(
            server_instance,
            service_registry
        );
        login_server.listen().await;
    };

    let login_handler = thread::spawn(move || {
        let rt = Runtime::new().unwrap();
        rt.block_on(login_server());
    });

    login_handler.join().unwrap();
}
