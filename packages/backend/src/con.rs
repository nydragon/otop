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

    /* pub async fn run(self: &mut Self) {
        // Check if message is received from the client through the WebSocket
        self.send(
            GatewayEvent::Hello as u8,
            serde_json::json!({
                "v": GATEWAY_VERSION,
                "heartbeat_interval": GATEWAY_HEARTBEAT_INTERVAL,
            }),
        )
        .await;

        loop {
            if let Some(msg) = self.socket.recv().await {
                if let Ok(msg) = msg {
                    // Get the message and convert it to json
                    let msg = msg.to_text().unwrap();

                    if msg.is_empty() {
                        println!("Received an empty message from the client !");
                        return;
                    }

                    println!("Received a message from the client: {:?}", msg);
                    let json = serde_json::from_str(msg);

                    if json.is_err() {
                        println!("Could not parse json");
                        return;
                    }

                    let json: serde_json::Value = json.unwrap();
                    let code = json["op"].as_i64().unwrap();

                    let op = code as i64;
                    match op {
                        // Handle the Heartbeat event
                        op if op == GatewayEvent::Heartbeat as i64 => {
                            self.last_heartbeat = SystemTime::now()
                                .duration_since(std::time::UNIX_EPOCH)
                                .unwrap()
                                .as_secs();
                            println!("Received a heartbeat from the client !");
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
    } */
    /*
    pub fn handle(self: &mut Self) {

        // Thread that will check every second if heartbeat is received
        loop {
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            // Get current timestamp
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();

            // Check if now > last_heartbeat + heartbeat_interval
            if self.last_heartbeat.is_none() || now < (self.last_heartbeat.unwrap() + self.heartbeat_interval) {
                continue;
            }
        }
    } */

    /* async fn read(self: &mut Self, mut receiver: SplitStream<WebSocket>) {
        while let Some(message) = receiver.next().await {
            let message = message.unwrap();

            // Get the message and convert it to json
            let msg = message.to_text().unwrap();

            if msg.is_empty() {
                println!("Received an empty message from the client !");
                return;
            }

            println!("Received a message from the client: {:?}", msg);
            let json = serde_json::from_str(msg);

            if json.is_err() {
                println!("Could not parse json");
                return;
            }

            let json: serde_json::Value = json.unwrap();
            let code = json["op"].as_i64().unwrap();

            let op = code as i64;
            match op {
                // Handle the Heartbeat event
                op if op == GatewayEvent::Heartbeat as i64 => {
                    println!("Received a heartbeat from the client !");
                }
                _ => {
                    println!("Received an unknown/illegal message from the client !");
                }
            }
        }
    } */

    /* async fn write(self: &mut Self, mut sender: SplitSink<WebSocket, Message>) {
        while self.open {
            while self.message_queue.len() > 0 {
                let (code, data) = self.message_queue.remove(0);
                let message = self.format_msg(code, data);

                if sender.send(message).await.is_err() {
                    println!("Failed to send message to client");
                    return;
                }
            }
        }
    } */

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

/* #[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_con_new() {
    }
} */
