mod con;
mod gateway;

use axum::extract::connect_info::ConnectInfo;
use axum::response::IntoResponse;
use axum::{
    extract::ws::WebSocketUpgrade,
    routing::get,
    Router,
};
use axum_extra::TypedHeader;
use headers::{self};
use once_cell::sync::Lazy;
use std::net::SocketAddr;

use crate::gateway::Gateway;

static GATEWAY: Lazy<Gateway> = Lazy::new(|| Gateway::new(42100, 100));

async fn ws_handler(
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
        GATEWAY.handle(socket, addr).await;
    })
}

#[tokio::main]
async fn main() {

    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route(
            "/ws",
            get(ws_handler),
        );

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
