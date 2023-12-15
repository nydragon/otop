use std::{sync::Arc, time::SystemTime};

use crate::con::Con;

use axum::extract::ws::WebSocket;
use tokio::sync::Mutex;

pub type Connection = (Arc<Mutex<WebSocket>>, Arc<Mutex<Con>>);

pub struct Gateway {
    //pub websockets: Vec<Arc<Mutex<Con>>>,
    pub connections: Vec<Connection>,
    pub max_connections: u64,
}

pub enum GatewayEvent {
    Heartbeat = 1,
    Data = 2,
    Hello = 10,
    HeartbeatAck = 11,
}

pub const GATEWAY_VERSION: u8 = 6;
pub const GATEWAY_HEARTBEAT_INTERVAL: u128 = 12 * 1000; // 12 seconds
pub const GATEWAY_DATA_INTERVAL: u128 = 1 * 1000; // 1 seconds

async fn launch_con(socket: Arc<Mutex<WebSocket>>, con: Arc<Mutex<Con>>) {
    con.lock()
        .await
        .send(
            socket.clone(),
            GatewayEvent::Hello as u8,
            serde_json::json!({
                "v": GATEWAY_VERSION,
                "heartbeat_interval": GATEWAY_HEARTBEAT_INTERVAL,
            }),
        )
        .await;

    loop {
        let con_open = con.lock().await.open;
        if !con_open {
            println!("Connection is closed, aborting.");
            break;
        }

        let msg = tokio::time::timeout(
            std::time::Duration::from_millis(100),
            socket.lock().await.recv(),
        )
        .await;

        if let Ok(msg) = msg {
            if let Some(Ok(msg)) = msg {
                // Get the message and convert it to json
                let msg = msg.to_text().unwrap();

                if msg.is_empty() {
                    con.lock().await.open = false;
                    println!("Connection is closed, aborting.");
                    break;
                }

                println!("Received a message from the client: {:?}", msg);
                let json = serde_json::from_str(msg);

                if json.is_err() {
                    println!("Could not parse json");
                    return;
                }

                let json: serde_json::Value = json.unwrap();
                let op = json["op"].as_i64().unwrap();

                println!("LHB: {}", con.lock().await.last_heartbeat);
                match op {
                    // Handle the Heartbeat event
                    op if op == GatewayEvent::Heartbeat as i64 => {
                        println!("Received a heartbeat from the client !");
                        let t = SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_millis();
                        con.lock().await.last_heartbeat = t;
                        con.lock()
                            .await
                            .send(
                                socket.clone(),
                                GatewayEvent::HeartbeatAck as u8,
                                serde_json::json!({
                                    "last_heartbeat": t
                                }),
                            )
                            .await;
                    }
                    _ => {
                        println!("Received an unknown/illegal message from the client !");
                    }
                }
            } else {
                println!("Received an illegal message from the client !");
                return;
            }
        }
    }
    println!("Client at {} has disconnected.", con.lock().await.addr);
}

impl Gateway {
    pub fn new(max_connections: u64) -> Self {
        Self {
            connections: Vec::new(),
            max_connections,
        }
    }

    pub async fn handle_connection(
        &mut self,
        socket: Arc<Mutex<WebSocket>>,
        addr: std::net::SocketAddr,
    ) {
        println!("Handling connection...");

        for (_, con) in &self.connections {
            if con.lock().await.addr == addr {
                println!("Connection already exists, aborting.");
                return;
            }
        }

        if self.connections.len() >= self.max_connections as usize {
            println!("Maximum number of connections reached, aborting.");
            return;
        }

        println!("New connection from {}", addr);
        let con = Arc::new(Mutex::new(Con::new(addr)));
        let con_clone = con.clone();

        self.connections.push((socket.clone(), con_clone));
        tokio::spawn(launch_con(socket, con));
        println!("Pushing connection to the list...");
    }
}
