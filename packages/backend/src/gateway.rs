use std::time::Duration;

use crate::con::Con;

use axum::extract::ws::WebSocket;

pub struct Gateway {
    pub connections: Vec<Con>,
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
        // Search for the connection with the same id
        let con = self.connections.iter().find(|c| c.addr == addr);

        if !con.is_none() || self.connections.len() >= self.max_connections as usize {
            return;
        }

        println!("New connection from {}", addr);
        let mut con = Con::new(socket, addr);

        // ===== Hello =====

        con.send(
            GatewayEvent::Hello as u8,
            serde_json::json!({
                "v": GATEWAY_VERSION,
                "heartbeat_interval": GATEWAY_HEARTBEAT_INTERVAL,
            }),
        )
        .await;

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
            self.connections
                .retain(|c| (c.last_heartbeat + GATEWAY_HEARTBEAT_INTERVAL) > current_time);
            // ===== Data =====
            for con in &mut self.connections {
                if con.open {
                    if (con.last_time_data_sent + GATEWAY_DATA_INTERVAL) < current_time {
                        // Send data to client
                        con.last_time_data_sent = current_time;
                    }
                } else {
                    con.run().await;
                    con.open = true;
                }
            }
        }
    }
}
