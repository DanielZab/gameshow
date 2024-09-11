use std::borrow::{Borrow, BorrowMut};
use std::future::IntoFuture;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use futures::lock::Mutex;
use serde::{Deserialize, Serialize};
use once_cell::sync::Lazy;
use futures::stream::{SplitSink, SplitStream};
use warp::filters::host;
use warp::Filter;
use warp::ws::{Message, WebSocket};
use futures::{FutureExt, SinkExt, StreamExt};

static HAS_HOST: AtomicBool = AtomicBool::new(false);

#[tokio::main]
async fn main() {
    // Define the WebSocket route
    let ws_route = warp::path("ws")
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| {
            ws.on_upgrade(handle_connection)
        });

    // Start the WebSocket server
    warp::serve(ws_route)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

#[derive(Debug, Clone, Copy)]
enum Role {
    Player,
    Host
}

struct PlayerRegistration {
    name: String,
    role: Role,
    tx: Option<SplitSink<WebSocket, Message>>,
    rx: Option<SplitStream<WebSocket>>
}


impl PlayerRegistration {
    fn new(name: String, role: Role) -> PlayerRegistration {
        PlayerRegistration{name, role, tx: None, rx: None}
    }

    async fn send(&mut self, text: &str) -> Result<(), String>{
        if self.tx.as_mut().unwrap().send(Message::text(text)).await.is_err() {
            return Err(format!("Error while writing {}", text));
        }
        return Ok(());
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct UserTemp {
    name: String,
    role: String
}

impl TryFrom<&str> for PlayerRegistration {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {

        let user: UserTemp = serde_json::from_str(value).map_err(|e| e.to_string())?;

        return match user.role.as_str() {
            "player" => Ok(PlayerRegistration::new(user.name, Role::Player)),
            "host" => Ok(PlayerRegistration::new(user.name, Role::Host)),
            _ => Err("Invalid role".into())
        }
    }
}

async fn handle_connection(ws: WebSocket) {
    let (mut tx, mut rx) = ws.split();

    // Respond to messages received from the client
    'outer: while let Some(result) = rx.next().await {
        if let Ok(msg) = result {
            if let Ok(text) = msg.to_str() {
                println!("Received: {}", text);

                if let Ok(mut registration) = PlayerRegistration::try_from(text) {
                    registration.rx = Some(rx);
                    registration.tx = Some(tx);
                    match registration.role {
                        Role::Host => {

                            if HAS_HOST.fetch_update(Ordering::Relaxed, Ordering::Relaxed, |x| Some(true)).unwrap() {
                                if let Err(e) = registration.send("Error, there is already a host").await {
                                    println!("{}", e);
                                    return;
                                }
                                rx = registration.rx.unwrap();
                                tx = registration.tx.unwrap();
                                continue 'outer;
                            }

                            if let Err(e) = registration.send("Ok").await {
                                println!("{}", e);
                                return;
                            }
                            match handle_host(registration).await {
                                Err(e) => println!("Error in host: {}", e),
                                _ => println!("Connection lost!") 
                            }
                        },
                        Role::Player => {
                            if let Err(e) = registration.send("Ok").await {
                                println!("{}", e);
                                return;
                            }
                            match handle_player(registration).await {
                                Err(e) => println!("Error in player: {}", e),
                                _ => println!("Connection lost!")
                            }
                        }
                    };
                    return;
                }

                // Echo the received message back to the client
                //if tx.send(Message::text(format!("Echo: {}", text))).await.is_err() {
                //    break;
               // }
            }
        }
    }
}

async fn handle_player(mut player: PlayerRegistration) -> Result<(), String> {
    let rx = player.rx.as_mut();
    match rx {
        None => return Err(String::from("No rx found")),
        _ => ()
    }
    let rx = rx.unwrap();
    while let Some(result) = rx.next().await {
        if let Ok(msg) = result {
            if let Ok(text) = msg.to_str() {
                println!("Player wrote: {}", text);
            }
        }
    }

    //connection.write("testste");

    Ok(())
}

async fn handle_host(mut player: PlayerRegistration) -> Result<(), String> {
    let rx = player.rx.as_mut();
    match rx {
        None => return Err(String::from("No rx found")),
        _ => ()
    }
    let rx = rx.unwrap();

    while let Some(result) = rx.next().await {
        if let Ok(msg) = result {
            if let Ok(text) = msg.to_str() {
                println!("Host wrote: {}", text);
            }
        }
    }

    Ok(())
}
/*
struct Player {
    connection: Connection
}

struct Admin {
    connection: Connection
}

struct Connection {
    reader: BufReader<TcpStream>,
    writer: BufWriter<TcpStream>
}

impl Connection {
    fn read_line(&mut self) -> Result<String, Error> {
        let mut buffer = String::from("");
        self.reader.read_line(&mut buffer)?;
        Ok(buffer)
    }

    fn read_all(&mut self) -> Result<String, Error> {
        let mut buffer = Vec::<u8>::new();
        self.reader.by_ref().lines().map(|a| a.unwrap()).reduce(|a, b| a + &b);
        Ok(String::from_utf8(buffer).map_err(|err| Error::new(ErrorKind::Other, String::from(err.to_string())))?)
    }

    fn new(stream: TcpStream) -> Connection {
        let stream_clone = stream.try_clone().unwrap();
        let reader = BufReader::new(stream_clone);
        let writer = BufWriter::new(stream);
        Connection{reader, writer}
    }

    fn write(&mut self, text: &str) -> Result<(), Error> {
        self.writer.write_all(text.as_bytes())?;
        self.writer.flush()?;
        Ok(())
    }
} */
