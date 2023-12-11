use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use crate::con::Con;

use axum::extract::ws::WebSocket;

pub struct Gateway {
    pub connections: Vec<Arc<Mutex<Con>>>,
    pub max_connections: u64,
}

pub enum GatewayEvent {
    Dispatch = 0,
    Heartbeat = 1,
    Data = 2,
    Hello = 10,
    HeartbeatAck = 11,
}

pub const GATEWAY_VERSION: u8 = 6;
pub const GATEWAY_HEARTBEAT_INTERVAL: u64 = 42000; // 42 seconds
pub const GATEWAY_DATA_INTERVAL: u64 = 120000; // 120 seconds

impl Gateway {
    pub fn new(max_connections: u64) -> Self {
        Self {
            connections: Vec::new(),
            max_connections,
        }
    }

    pub async fn handle_connection(self: &mut Self, socket: WebSocket, addr: std::net::SocketAddr) {
        println!("Handling connection...");
        let is_existent = self
            .connections
            .iter().find(|c| {
                if let Ok(c) = c.lock() {
                    c.addr == addr
                } else {
                    false
                }
            });

        println!("Checking if connection is existent. {}", is_existent.is_some());
        if !is_existent.is_none() || self.connections.len() >= self.max_connections as usize {
            return;
        }

        println!("New connection from {}", addr);
        let con = Arc::new(Mutex::new(Con::new(socket, addr)));
        let con_clone = con.clone();

        std::thread::spawn(move || {
            tokio::runtime::Runtime::new().unwrap().block_on(async move {
                println!("Running connection...");
                con_clone.lock().unwrap().run().await;
            });
        });

        print!("Pushing connection to the list");
        self.connections.push(con);

        //self.connections.retain(|c| c.open);
    }

    pub async fn run(self: &mut Self) {
        loop {
            tokio::time::sleep(Duration::from_millis(100)).await;
            let current_time = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64;
            // ===== Heartbeat =====
            self.connections.retain(|c| match c.lock() {
                Ok(c) => {
                    println!("Client at {} has not responded to heartbeat, removing.", c.addr);
                    (c.last_heartbeat + GATEWAY_HEARTBEAT_INTERVAL) < current_time
                },
                Err(_) => {
                    eprintln!("Connection is poisoned, removing.");
                    false
                }
            });

            // ===== Data =====
            for con in &mut self.connections {
                if let Ok(mut con) = con.lock() {
                    if (con.last_time_data_sent + GATEWAY_DATA_INTERVAL) < current_time {
                        // Send data to client
                        println!("Client at {} will receive data.", con.addr);
                        con.last_time_data_sent = current_time;
                    }
                } else {
                    eprintln!("Connection is poisoned, skipping.");
                }
            }
        }
    }
}
