mod config;
pub mod filetype;
mod routes;
mod websocket;
use axum::{
    Router,
    routing::{get, get_service},
};
use config::*;
use protocol::UserMessage;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::broadcast;
use tower_http::services::ServeDir;

#[derive(Clone)]
struct AppState {
    sender: Arc<broadcast::Sender<UserMessage>>,
    config: Config,
}

#[tokio::main]
async fn main() {
    // Shared broadcast channel with a buffer of 100 messages
    let (tx, _) = broadcast::channel::<UserMessage>(100);
    let state = AppState {
        sender: Arc::new(tx),
        config: load_config("./Server.toml").expect("Failed to load config"),
    };

    let app = Router::new()
        .route("/ws", get(websocket::handler))
        .route("/info", get(routes::info::handler))
        .nest_service("/images", get_service(ServeDir::new("./assets/images/")))
        .with_state(state.clone());

    let addr = SocketAddr::from((state.config.server.host, state.config.server.port));
    println!("Listening on {}", addr);
    axum::serve(
        tokio::net::TcpListener::bind(addr).await.unwrap(),
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}
