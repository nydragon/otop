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

        con.send(GatewayEvent::Hello as u8, serde_json::json!({
            "v": GATEWAY_VERSION,
            "heartbeat_interval": GATEWAY_HEARTBEAT_INTERVAL,
        })).await;

        self.connections.push(con);
        //self.connections.retain(|c| c.open);
    }

    pub async fn run(self: &mut Self) {
        // TODO: Handle three threads (one to verify heartbeat, one to send data, one to handle client messages) using tokio
        // 1. The first thread will check every second if heartbeat is received in time (last_heartbeat + heartbeat_interval) < GATEWAY_HEARTBEAT_INTERVAL 
        // 2. The second thread will send data to the client every DATA_INTERVAL seconds
        // 3. The third thread will handle messages from the client

        let heartbeat_sender = tokio::spawn(self.heartbeat_handler());
        let data_sender = tokio::spawn(self.data_sender());
        //let message_handler = tokio::spawn(self.message_handler());

        tokio::try_join!(heartbeat_sender, data_sender).unwrap();
    }

    async fn heartbeat_handler(self: &mut Self) {
        loop {
            tokio::time::sleep(Duration::from_millis(100)).await;
            let current_time = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() as u64;
            for con in &mut self.connections {
                if (con.last_heartbeat + GATEWAY_HEARTBEAT_INTERVAL) < current_time {
                    println!("Client {} timed out", con.addr);
                    con.open = false;
                }
            }
        }
    }

    async fn data_sender(self: &mut Self) {

        loop {

            // TODO: Implement logic to send data to connected clients
            // For example, you can iterate over self.connections and send data to each client.
            tokio::time::sleep(Duration::from_millis(100)).await;
            let current_time = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() as u64;
            for con in &mut self.connections {
                if (con.last_time_data_sent + GATEWAY_DATA_INTERVAL) < current_time {
                    // Send data to client
                    con.last_time_data_sent = current_time;
                }
            }
        }
    }

    async fn message_handler(self: &mut Self) {
        // TODO: Implement logic to handle messages from connected clients
        // For example, you can use a loop to continuously receive and handle messages from the clients.
    }

}
