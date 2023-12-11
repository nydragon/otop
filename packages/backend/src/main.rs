mod con;
mod gateway;

use axum::extract::connect_info::ConnectInfo;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::{extract::ws::WebSocketUpgrade, routing::get, Router};
use axum_extra::TypedHeader;
use headers::{self};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::gateway::Gateway;

async fn ws_handler(
    State(gateway): State<Arc<Mutex<Gateway>>>,
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
        user_agent.to_string()
    } else {
        String::from("Unknown browser")
    };
    println!("`{}` at {} connected.", user_agent, addr);

    // finalize the upgrade process by returning upgrade callback.
    // we can customize the callback by sending additional info such as address.
    ws.on_upgrade(move |socket| async move {
        gateway.lock().await.handle_connection(socket, addr).await;
    })
}

#[tokio::main]
async fn main() {
    let gateway = Arc::new(Mutex::new(Gateway::new(100)));

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
