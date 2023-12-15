use crate::con::Con;
use axum::extract::ws::WebSocket;
use std::{sync::Arc, time::SystemTime};
use tokio::sync::Mutex;

pub type Connection = (Arc<Mutex<WebSocket>>, Arc<Mutex<Con>>);

pub struct Gateway {
    pub connections: Vec<Connection>,
    pub max_connections: u64,
}

pub enum GatewayEvent {
    Heartbeat = 1,
    Data = 2,
    Terminate = 3,
    Hello = 10,
    HeartbeatAck = 11,
    Unknown = 99,
}

#[derive(serde::Deserialize)]
struct Terminate {
    pid: i32,
    signal: i32,
}

impl From<i64> for GatewayEvent {
    fn from(value: i64) -> Self {
        match value {
            1 => GatewayEvent::Heartbeat,
            2 => GatewayEvent::Data,
            3 => GatewayEvent::Terminate,
            10 => GatewayEvent::Hello,
            11 => GatewayEvent::HeartbeatAck,
            _ => GatewayEvent::Unknown,
        }
    }
}

pub const GATEWAY_VERSION: u8 = 6;
pub const GATEWAY_HEARTBEAT_INTERVAL: u128 = 12 * 1000; // 12 seconds
pub const GATEWAY_DATA_INTERVAL: u128 = 1000; // 1 seconds

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
            log::info!("Connection is closed, aborting.");
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
                    log::info!("Connection is closed, aborting.");
                    break;
                }

                log::info!("Received a message from the client: {:?}", msg);
                let json = serde_json::from_str(msg);

                if json.is_err() {
                    log::error!("Could not parse json");
                    return;
                }

                let json: serde_json::Value = json.unwrap();
                let op = json["op"].as_i64().unwrap();

                log::debug!("LHB: {}", con.lock().await.last_heartbeat);
                match GatewayEvent::from(op) {
                    // Handle the Heartbeat event
                    GatewayEvent::Heartbeat => {
                        log::info!("Received a heartbeat from the client !");
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
                    GatewayEvent::Terminate => {
                        if let Ok(res) = serde_json::from_str::<Terminate>(&json["d"].to_string()) {
                            kill_process(res.pid, res.signal);
                        } else {
                            log::error!("Data has an incorrect structure");
                        }
                    }
                    _ => {
                        log::error!("Received an unknown/illegal message from the client !");
                    }
                };
            } else {
                log::error!("Received an illegal message from the client !");
            }
        }
    }
    log::info!("Client at {} has disconnected.", con.lock().await.addr);
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
        log::info!("Handling connection...");

        for (_, con) in &self.connections {
            if con.lock().await.addr == addr {
                log::warn!("Connection already exists, aborting.");
                return;
            }
        }

        if self.connections.len() >= self.max_connections as usize {
            log::warn!("Maximum number of connections reached, aborting.");
            return;
        }

        log::info!("New connection from {}", addr);
        let con = Arc::new(Mutex::new(Con::new(addr)));
        let con_clone = con.clone();

        self.connections.push((socket.clone(), con_clone));
        tokio::spawn(launch_con(socket, con));
        log::info!("Pushing connection to the list...");
    }
}

/// Send an interrupt signal to a specified process.
fn kill_process(pid: i32, sig: i32) {
    unsafe {
        libc::kill(pid, sig);
    };
}
