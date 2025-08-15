use crate::SecurityMode;
use crate::socket::handler::handle_connection;
use crate::socket::security::load_tls_config;
use crate::server::Server;
use std::{
    fmt::Error, sync::{Arc, Mutex}
};

use tokio::{
    net::{TcpListener},
};
use tokio_rustls::TlsAcceptor;

use crate::socket::handler::handle_connection_sec;

pub async fn listen(mode: SecurityMode, server: Server) -> Result<(), Error> {
    
    let tcp_listener = TcpListener::bind("0.0.0.0:9001").await.expect("Bind failed");

    if mode == SecurityMode::Secure {
        let config = load_tls_config();
        let acceptor = TlsAcceptor::from(Arc::new(config));


        println!("WSS server running on wss://localhost:9001");

        while let Ok((tcp_stream, addr)) = tcp_listener.accept().await {
            let acceptor = acceptor.clone();
            let server_clone = server.clone();

            tokio::spawn(async move {
                match acceptor.accept(tcp_stream).await {
                    Ok(tls_stream) => {
                        handle_connection_sec(tls_stream, addr, server_clone).await;
                    }
                    Err(e) => {
                        eprintln!("TLS error: {:?}", e);
                    }
                }
            });
        }
    } else {
        println!("WS server running on ws://localhost:9001");

        while let Ok((tcp_stream, addr)) = tcp_listener.accept().await {
            let server_clone = server.clone();

            tokio::spawn(async move {
                handle_connection(tcp_stream, addr, server_clone).await;
            });
        }
    }
    Ok(())

}