use crate::*;
use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::Response,
};
use futures_util::{SinkExt, StreamExt};
use protocol::UserMessage;
use std::sync::Arc;
use tokio::sync::broadcast;

pub async fn handler(
    ws: WebSocketUpgrade,
    axum::extract::State(state): axum::extract::State<AppState>,
) -> Response {
    println!("Upgrading connection");
    ws.on_upgrade(move |socket| handle_socket(socket, state.sender))
}

async fn handle_socket(socket: WebSocket, tx: Arc<broadcast::Sender<UserMessage>>) {
    let mut rx = tx.subscribe();

    let (mut sender, mut receiver) = socket.split();

    // Task to receive from WebSocket and broadcast to others
    let tx_send = tx.clone();
    let send_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            if let Message::Text(text) = msg {
                println!("Raw message: {:#?}", text);
                match serde_json::from_str::<UserMessage>(&text) {
                    Ok(user_msg) => {
                        println!("Received: {:?}", user_msg);
                        let _ = tx_send.send(user_msg);
                    }
                    Err(e) => {
                        eprintln!("Failed to parse message: {}", e);
                    }
                }
            }
        }
    });

    // Task to send broadcasted messages to this WebSocket client

    let recv_task = tokio::spawn(async move {
        while let Ok(user_msg) = rx.recv().await {
            match serde_json::to_string(&user_msg) {
                Ok(json) => {
                    if sender.send(Message::Text(json.into())).await.is_err() {
                        break;
                    }
                }
                Err(e) => {
                    eprintln!("Failed to serialize message: {}", e);
                }
            }
        }
    });

    // Wait for either task to finish (disconnect, etc.)
    tokio::select! {
        _ = send_task => (),
        _ = recv_task => (),
    }

    println!("Client disconnected");
}
