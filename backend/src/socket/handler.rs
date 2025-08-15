use tokio_rustls::server::TlsStream;
use std::{
    net::SocketAddr,
};
use futures::{SinkExt, StreamExt};
use tokio::{
    net::{TcpStream},
    sync::mpsc::{self},
};
use tokio_tungstenite::{accept_async, tungstenite::protocol::Message};
use uuid::Uuid;
use super::super::Server;


pub async fn handle_connection_sec(stream: TlsStream<TcpStream>, addr: SocketAddr, server: Server) {
    let ws_stream = accept_async(stream).await.expect("WebSocket handshake failed");
    println!("New WebSocket connection from {}", addr);

    let (mut ws_sender, mut ws_receiver) = ws_stream.split();

    let (tx, mut rx) = mpsc::unbounded_channel::<Message>();
    let client_id = Uuid::new_v4();
    server.register_client(client_id, tx.clone());

    // Task to write to the client
    let write_to_client = async {
        while let Some(msg) = rx.recv().await {
            if ws_sender.send(msg).await.is_err() {
                break;
            }
        }
    };

    // Task to read from the client
    let read_from_client = async {
        while let Some(Ok(msg)) = ws_receiver.next().await {
            if msg.is_text() || msg.is_binary() {
                let to_send = format!("Received from {}: {:?}", addr, msg);
                server.update_state(to_send);
                server.broadcast_state();
            }
        }
    };

    tokio::select! {
        _ = write_to_client => {},
        _ = read_from_client => {},
    }

    println!("Connection closed: {}", addr);
    server.unregister_client(&client_id);

}

pub async fn handle_connection(stream: TcpStream, addr: SocketAddr, server: Server) {
    let ws_stream = accept_async(stream).await.expect("WebSocket handshake failed");
    println!("New WebSocket connection from {}", addr);

    let (mut ws_sender, mut ws_receiver) = ws_stream.split();

    let (tx, mut rx) = mpsc::unbounded_channel::<Message>();
    let client_id = Uuid::new_v4();
    server.register_client(client_id, tx.clone());

    // Task to write to the client
    let write_to_client = async {
        while let Some(msg) = rx.recv().await {
            let msg_clone = msg.clone();
            if ws_sender.send(msg).await.is_err() {
                println!("Aborting");
                break;
            } else {
                println!("Sent {}", &msg_clone);
            }
        }
    };

    // Task to read from the client
    let read_from_client = async {
        while let Some(Ok(msg)) = ws_receiver.next().await {
            if msg.is_text() || msg.is_binary() {
                let to_send = format!("Received from {}: {:?}", addr, msg);
                println!("{}", &to_send);
                server.update_state(to_send);
                server.broadcast_state();
            }
        }
    };

    tokio::select! {
        _ = write_to_client => {},
        _ = read_from_client => {},
    }

    println!("Connection closed: {}", addr);
    server.unregister_client(&client_id);

}