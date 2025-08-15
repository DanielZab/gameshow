pub mod socket;
pub mod rest;
pub mod server;
pub mod constants;

use socket::listener::listen;
use server::Server;
use rest::listener::start_rest_listener;

#[derive(PartialEq)]
pub enum SecurityMode {
    Insecure,
    Secure
}


#[actix_rt::main]
async fn main() {
    let server = Server::new();
    {
       let mode = SecurityMode::Insecure;
        let server_clone = server.clone();
        tokio::spawn(async move {
            if let Err(e) = listen(mode, server_clone).await {
                eprintln!("WebSocket listener failed: {}", e);
            }
        });
    }
    start_rest_listener().await;
}
