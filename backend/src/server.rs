use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use serde::{Deserialize, Serialize};
use tokio::{
    sync::mpsc::{UnboundedSender},
};
use tokio_tungstenite::{tungstenite::protocol::Message};
use uuid::Uuid;


#[derive(Debug, Serialize, Deserialize, Clone)]
struct SharedState {
    counter: usize,
    message: String,
}

#[derive(Clone)]
pub struct Server {
    state: Arc<Mutex<SharedState>>,
    clients: Arc<Mutex<HashMap<Uuid, UnboundedSender<Message>>>>,
}

impl Server {
    pub fn new() -> Self {
        Server {
            state: Arc::new(Mutex::new(SharedState { counter: 0, message: String::new()})),
            clients: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn broadcast_state(&self) {
        let state = self.state.lock().unwrap().clone();
        let message = Message::Text(serde_json::to_string(&state).unwrap());
        let clients = self.clients.lock().unwrap();

        for tx in clients.values() {
            let _ = tx.send(message.clone());
        }
    }

    pub fn update_state(&self, msg: String) {
        let mut state = self.state.lock().unwrap();
        state.counter += 1;
        state.message = msg;        
    }

    pub fn register_client(&self, id: Uuid, tx: UnboundedSender<Message>) {
        self.clients.lock().unwrap().insert(id, tx);
    }

    pub fn unregister_client(&self, id: &Uuid) {
        self.clients.lock().unwrap().remove(id);
    }
}