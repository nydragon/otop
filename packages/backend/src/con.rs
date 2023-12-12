use std::{sync::Arc, time::SystemTime};

use axum::{self, extract::ws::WebSocket};
use tokio::sync::Mutex;

pub struct Con {
    pub addr: std::net::SocketAddr, // The address of the client
    pub last_heartbeat: u64,        // The last heartbeat received from the client
    pub last_time_data_sent: u64,   // The last time data was sent to the client
    pub open: bool,
}

impl Con {
    pub fn new(addr: std::net::SocketAddr) -> Self {
        Self {
            addr,
            last_heartbeat: 0,
            last_time_data_sent: 0,
            open: true,
        }
    }

    pub async fn send(&mut self, socket: Arc<Mutex<WebSocket>>, code: u8, data: serde_json::Value) {
        let json: serde_json::Value = serde_json::json!({
            "op": code,
            "d": data
        });

        let message = axum::extract::ws::Message::Text(json.to_string());
        socket
            .lock()
            .await
            .send(message)
            .await
            .expect("ALARM ALARM");
    }
}