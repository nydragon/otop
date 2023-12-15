mod con;
mod gateway;

use crate::gateway::{Gateway, GatewayEvent, GATEWAY_HEARTBEAT_INTERVAL};
use ::log as ext_log;
use axum::extract::connect_info::ConnectInfo;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::{extract::ws::WebSocketUpgrade, routing::get, Router};
use axum_extra::TypedHeader;
use clap::Parser;
use gateway::GATEWAY_DATA_INTERVAL;
use headers::{self};
use process::data::Data;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;

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
    log::info!("`{}` at {} connected.", user_agent, addr);

    // finalize the upgrade process by returning upgrade callback.
    // we cacn customize the callback by sending additional info such as address.
    ws.on_upgrade(move |socket| async move {
        log::info!("Client at `{}` start to handle connection.", addr);

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
            .as_millis();
        // ===== Heartbeat =====
        let mut remove_idxs: Vec<usize> = Vec::new();
        log::debug!(
            "Gateway has {} connections.",
            gateway.lock().await.connections.len()
        );
        log::debug!("Current Time: {}", current_time);
        for (i, con) in gateway.lock().await.connections.iter().enumerate() {
            match con.1.try_lock() {
                Ok(mut c) => {
                    if !c.open {
                        log::info!("Connection is closed, removing.");
                        remove_idxs.push(i);
                        continue;
                    }

                    log::debug!(
                        "Checking heartbeat for client at ({}), last heartbeat was {}s ago",
                        c.addr,
                        ((current_time - c.last_heartbeat) / 1000) as f32
                    );
                    let diff = current_time - c.last_heartbeat;
                    if c.last_heartbeat > 0 && diff > GATEWAY_HEARTBEAT_INTERVAL {
                        // Remove the connection
                        log::warn!(
                            "Client at {} has not responded to heartbeat, removing.",
                            c.addr
                        );
                        remove_idxs.push(i);
                    } else {
                        if c.last_heartbeat == 0 {
                            c.last_heartbeat = current_time;
                        }
                        // Send heartbeat
                        log::debug!("Waiting heartbeat from client at {}....", c.addr);
                    }
                }
                Err(_) => {
                    log::warn!("Connection is poisoned, removing.");
                    remove_idxs.push(i);
                }
            }
        }

        let r_len = remove_idxs.len();

        for i in remove_idxs {
            gateway
                .lock()
                .await
                .connections
                .get(i)
                .unwrap()
                .1
                .lock()
                .await
                .open = false;
            gateway.lock().await.connections.remove(i);
        }

        if r_len != 0 {
            gateway.lock().await.log_con_n();
        }

        // ===== Data =====
        log::debug!("Preparing to send data...");
        for (socket, con) in &gateway.lock().await.connections {
            match con.try_lock() {
                Ok(mut c) => {
                    let diff = current_time - c.last_time_data_sent;
                    if diff > GATEWAY_DATA_INTERVAL {
                        // Send data to client
                        log::debug!("Client at {} will receive data.", c.addr);

                        c.send(
                            socket.clone(),
                            GatewayEvent::Data as u8,
                            serde_json::json!(Data::new()),
                        )
                        .await;

                        c.last_time_data_sent = current_time;
                    } else {
                        // Don't send data to client
                        log::debug!("Client at {} will not receive data.", c.addr);
                    }
                }
                Err(_) => {
                    log::error!("Connection is poisoned.");
                }
            }
        }
    }
}

#[derive(Parser, Debug)]
struct Args {
    #[clap(
        long,
        short,
        help = "Enable a more detailed level of logs.",
        default_value_t = false
    )]
    pub verbose: bool,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let mut builder = colog::builder();

    if args.verbose {
        builder.filter_level(ext_log::LevelFilter::Debug);
    }

    builder.init();

    log::debug!("Debug mode activated");
    log::debug!("{:#?}", args);

    let gateway = Arc::new(Mutex::new(Gateway::new(5)));
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
    log::info!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}
