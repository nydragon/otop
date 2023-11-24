use axum::{routing::get, Router, extract::ws::{WebSocketUpgrade, WebSocket, Message}, response::Response, Error};
use futures_util::{sink::SinkExt, stream::{StreamExt, SplitSink, SplitStream}};

async fn handler(
    ws: WebSocketUpgrade
) -> Response {
    ws.on_upgrade(move |socket| handle_socket(socket))
}

async fn do_handshake(mut sender : SplitSink<WebSocket, Message>, message: Result<Message, Error>) -> bool {
    let message = message.unwrap();

    // Get the message and convert it to json
    let msg = message.to_text().unwrap();
    let json= serde_json::from_str(msg);

    if json.is_err() {
        return false;
    }

    let json : serde_json::Value = json.unwrap();

    if json["op"].as_i64().unwrap() != 1 {
        return false;
    }

    let ping_response = Message::Text("{\"op\": 11 }".to_string());

    if sender.send(ping_response).await.is_ok() {
        return true;
    }

    false
}

async fn handle_socket(socket: WebSocket) {

    // Create a json as follo { "op": 10 }
    let hello_msg = Message::Text(format!("{{\"op\": 10, \"d\": {{\"heartbeat_interval\": {}}}}}", 42100).to_string());

    let (mut sender, mut receiver) = socket.split();
    
    // Hello message
    if sender.send(hello_msg).await.is_ok() {
        println!("Pinged...");
    } else {
        println!("Could not send ping !");
        return;
    }

    // Wait for the first heartbeat ack within 5 seconds
    let message = tokio::time::timeout(std::time::Duration::from_secs(5), receiver.next()).await;
    if message.is_err() {
        println!("Did not receive a heartbeat ack within 5 seconds !");
        return;
    }

    let message = message.unwrap();
    if message.is_none() {
        println!("Did not receive a heartbeat ack within 5 seconds !");
        return;
    }

    do_handshake(sender, message.unwrap()).await;
/* 
    tokio::spawn(read(receiver));
    tokio::spawn(write(sender)); */
}

async fn read(mut receiver: SplitStream<WebSocket>) {
    while let Some(message) = receiver.next().await {
        let message = message.unwrap();

        // Get the message and convert it to json
        let msg = message.to_text().unwrap();

        if msg.is_empty() {
            println!("Received an empty message from the client !");
            return;
        }

        println!("Received a message from the client: {:?}", msg);
        let json= serde_json::from_str(msg);

        if json.is_err() {
            println!("Could not parse json");
            return;
        }

        let json : serde_json::Value = json.unwrap();

        match json["op"].as_i64().unwrap() {
            1 => {
                println!("Received a heartbeat from the client !");
            },
            10 => {
                println!("Received a hello message from the client !");
            },
            11 => {
                println!("Received a heartbeat ack from the client !");
            },
            _ => {
                println!("Received an unknown message from the client !");
            }
        }
    }
}

/* async fn write(mut sender: SplitSink<WebSocket, Message>) {
    let message = Message::Text("Hello from the server!".to_string());
    sender.send(message).await.unwrap();
}
 */
#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/ws", get(handler))
    ;

    // run it with hyper on localhost:3000
    let listener = tokio::net::TcpListener::bind(&"0.0.0.0:3000")
        .await
        .unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());
    axum::Server::from_tcp(listener.into_std().unwrap())
        .unwrap()
        .serve(app.into_make_service())
        .await
        .unwrap();
}
