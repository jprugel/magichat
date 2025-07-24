use axum::extract::{
    State,
    Json,
};
use axum::http::StatusCode;
use protocol::user::{CreateUserRequest, UserRepository};
use crate::AppState;

/// Should allow crud for users
pub async fn create_user(
    Json(create_user_request): Json<CreateUserRequest>,
    State(state): State<AppState>
) -> StatusCode {
    let response = state.database
        .create_user(&create_user_request)
        .await;
    
    match response {
        Ok(user) => StatusCode::ACCEPTED,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}