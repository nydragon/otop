mod con;
mod gateway;

use axum::extract::connect_info::ConnectInfo;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::{extract::ws::WebSocketUpgrade, routing::get, Router};
use axum_extra::TypedHeader;
use gateway::GATEWAY_DATA_INTERVAL;
use headers::{self};
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;

use crate::gateway::{Gateway, GATEWAY_HEARTBEAT_INTERVAL};

async fn ws_handler(
    State(gateway): State<Arc<Mutex<Gateway>>>,
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
        user_agent.to_string()
    } else {
        String::from("UB")
    };
    println!("`{}` at {} connected.", user_agent, addr);

    // finalize the upgrade process by returning upgrade callback.
    // we cacn customize the callback by sending additional info such as address.
    ws.on_upgrade(move |socket| async move {
        println!("Client at `{}` start to handle connection.", addr);

        let socket = Arc::new(Mutex::new(socket));

        gateway.lock().await.handle_connection(socket, addr).await;
    })
}

async fn run(gateway: Arc<Mutex<Gateway>>) {
    loop {
        tokio::time::sleep(Duration::from_millis(500)).await;
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;
        // ===== Heartbeat =====
        let mut remove_idxs: Vec<usize> = Vec::new();
        // Refactor above code because mutex is the tokio mutex
        let mut gateway = gateway.lock().await;
        println!("Gateway has {} connections.", gateway.connections.len());
        println!("Current Time: {}", current_time);
        for (i, con) in gateway.connections.iter().enumerate() {
            match con.1.try_lock() {
                Ok(mut c) => {
                    println!("Checking heartbeat for client at {}, last heartbeat {} was {}s ago", c.addr, c.last_heartbeat, ((current_time - c.last_heartbeat) / 1000) as f32);
                    let diff = current_time - c.last_heartbeat;
                    if c.last_heartbeat > 0 && diff > GATEWAY_HEARTBEAT_INTERVAL {
                        // Remove the connection
                        println!(
                            "Client at {} has not responded to heartbeat, removing.",
                            c.addr
                        );
                        // Close the websocket
                        remove_idxs.push(i);
                    } else {
                        if c.last_heartbeat == 0 {
                            c.last_heartbeat = current_time;
                        }
                        // Send heartbeat
                        println!("Waiting heartbeat from client at {}....", c.addr);
                    }
                }
                Err(_) => {
                    println!("Connection is poisoned, removing.");
                    //remove_idxs.push(i);
                }
            }
        }

        for i in remove_idxs {
            //gateway.connections.get(i).unwrap().0.lock().await.close();
            gateway.connections.remove(i);
        }

        // ===== Data =====
        println!("Preparing to send data...");
        for (_, con) in &gateway.connections {
            match con.try_lock() {
                Ok(mut c) => {
                    if (c.last_time_data_sent + GATEWAY_DATA_INTERVAL) < current_time {
                        // Send data to client
                        println!("Client at {} will receive data.", c.addr);
                        c.last_time_data_sent = current_time;
                    } else {
                        // Don't send data to client
                        println!("Client at {} will not receive data.", c.addr);
                    }
                }
                Err(_) => {
                    println!("Connection is poisoned, removing.");
                    //remove_idxs.push(i);
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let gateway = Arc::new(Mutex::new(Gateway::new(100)));
    //let mut gateway_clone = gateway.clone();

    tokio::spawn(run(gateway.clone()));
    // TODO: Call the gateway's run method to start the heartbeat and message handling loops in the background

    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/ws", get(ws_handler))
        .with_state(gateway);

    // run it with hyper on localhost:3000
    let listener = tokio::net::TcpListener::bind(&"0.0.0.0:3000")
        .await
        .unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}
