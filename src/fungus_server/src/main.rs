use std::thread;
use env_logger::Builder;
use fungus_login::acceptor::LoginServer;
use log::LevelFilter;
use tokio::runtime::Runtime;
use fungus_net::server::server::SERVER_INSTANCE;

#[tokio::main]
async fn main() {
    Builder::new().filter(None, LevelFilter::Info).init();

    // Load Server data
    {
        let _ = SERVER_INSTANCE.write().await;
    }

    let login_server = || async {
        let mut login_server = LoginServer::new();
        login_server.listen().await;
    };

    let login_handler = thread::spawn(move || {
        let rt = Runtime::new().unwrap();
        rt.block_on(login_server());
    });

    login_handler.join().unwrap();
}
