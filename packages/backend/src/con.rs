use crate::gateway::GatewayEvent;

use axum::{
    self,
    extract::ws::{Message, WebSocket},
};
use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};

pub struct Con {
    pub socket: WebSocket,           // The WebSocket connection
    pub addr: std::net::SocketAddr,  // The address of the client
    pub last_heartbeat: Option<u64>, // The last heartbeat received from the client
    pub heartbeat_interval: u64,     // The interval between two data messages to sent
    pub data_interval: u64,          // The interval between two data messages to sent
    pub message_queue: Vec<(GatewayEvent, serde_json::Value)>, // The message queue
    pub open: bool,
}

impl Con {
    pub fn new(
        socket: WebSocket,
        addr: std::net::SocketAddr,
        heartbeat_interval: u64,
        data_interval: u64,
    ) -> Self {
        Self {
            socket,
            addr,
            last_heartbeat: None,
            heartbeat_interval: heartbeat_interval,
            data_interval: data_interval,
            message_queue: Vec::new(),
            open: true,
        }
    }

    pub fn handle(self: &Self) {
        /*  let (mut sender, mut receiver) = self.socket.split();

        // Read and write messages
        let rth = tokio::spawn(self.read(receiver));
        let wth = tokio::spawn(self.write(sender));

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

            // Kill the connection (Stop the threads)
            rth.abort();
            wth.abort();
        } */
    }

    pub async fn hello(self: &mut Self, v: u8) {
        let msg = self.format_msg(
            GatewayEvent::Hello,
            serde_json::json!({
                "v": v,
                "heartbeat_interval": self.heartbeat_interval,
            }),
        );

        self.send(msg).await;
    }

    async fn read(self: &Self, mut receiver: SplitStream<WebSocket>) {
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
    }

    async fn write(self: &mut Self, mut sender: SplitSink<WebSocket, Message>) {
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
    }

    async fn send(self: &mut Self, message: Message) {
        if self.socket.send(message).await.is_err() {
            println!("Failed to send message to client");
            return;
        }
    }

    // Send a message to the client
    fn format_msg(self: &mut Self, code: GatewayEvent, data: serde_json::Value) -> Message {
        let json: serde_json::Value = serde_json::json!({
            "op": code as u8,
            "d": data
        });

        let message = axum::extract::ws::Message::Text(json.to_string());
        message
    }
}

unsafe impl Sync for Con {}

/* #[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_con_new() {
    }
} */
