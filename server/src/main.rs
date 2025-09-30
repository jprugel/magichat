mod config;
pub mod database;
pub mod filetype;
mod routes;
mod websocket;

use axum::routing::post;
use axum::{
    Router,
    routing::{get, get_service},
};
use config::*;
use database::*;
use protocol::{User, UserMessage};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::broadcast;
use tower_http::services::ServeDir;

#[derive(Clone)]
struct AppState {
    sender: Arc<broadcast::Sender<UserMessage>>,
    config: Config,
    users: Vec<User>,
    database: Database,
}

#[tokio::main]
async fn main() {
    // Shared broadcast channel with a buffer of 100 messages
    let (tx, _) = broadcast::channel::<UserMessage>(100);
    let state = AppState {
        sender: Arc::new(tx),
        config: load_config("./Server.toml").expect("Failed to load config"),
        users: Vec::new(),
        database: database::Database::builder()
            .pool(5)
            .url("postgres://postgres:password@database:5432/mydb")
            .build()
            .await
            .unwrap(),
    };

    let user_route = post(routes::user::create_user)
        .get(routes::user::read_user)
        .delete(routes::user::delete_user)
        .put(routes::user::update_user);

    let app = Router::new()
        .route("/ws", get(websocket::handler))
        .route("/info", get(routes::info::handler))
        .route("/user", user_route)
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
