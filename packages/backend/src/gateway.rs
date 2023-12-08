use crate::con::Con;

use axum::extract::ws::WebSocket;

pub struct Gateway {
    pub heartbeat_interval: u64,
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
pub const GATEWAY_HEARTBEAT_INTERVAL: u64 = 42100; // 42.1 seconds
pub const GATEWAY_DATA_INTERVAL: u64 = 120000; // 120 seconds

impl Gateway {
    pub fn new(heartbeat_interval: u64, max_connections: u64) -> Self {
        Self {
            heartbeat_interval,
            connections: Vec::new(),
            max_connections,
        }
    }

    pub async fn handle(self: &mut Self, socket: WebSocket, addr: std::net::SocketAddr) {
        // Search for the connection with the same id
        let con = self.connections.iter().find(|c| c.addr == addr);

        if !con.is_none() {
            return;
        }

        let mut con = Con::new(
            socket,
            addr,
            GATEWAY_HEARTBEAT_INTERVAL,
            GATEWAY_DATA_INTERVAL,
        );
        con.hello(GATEWAY_VERSION).await;

        con.handle();

        self.connections.push(con);
    }
}
