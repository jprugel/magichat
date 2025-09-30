use crate::AppState;
use crate::database::{Database, UserRepository};
use axum::extract::{Json, State};
use axum::http::StatusCode;
use protocol::user::{CreateUserRequest, DeleteUserRequest, ReadUserRequest, UpdateUserRequest};

/// Should allow crud for users
#[axum::debug_handler]
pub async fn create_user(
    State(state): State<AppState>,
    Json(create_user_request): Json<CreateUserRequest>,
) -> StatusCode {
    let response = state.database.create_user(&create_user_request).await;

    match response {
        Ok(user) => StatusCode::ACCEPTED,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

pub async fn read_user(
    State(state): State<AppState>,
    Json(read_user_request): Json<ReadUserRequest>,
) -> StatusCode {
    let response = state.database.read_user(&read_user_request).await;

    match response {
        Ok(user) => StatusCode::ACCEPTED,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

pub async fn update_user(
    State(state): State<AppState>,
    Json(update_user_request): Json<UpdateUserRequest>,
) -> StatusCode {
    let response = state.database.update_user(&update_user_request).await;

    match response {
        Ok(user) => StatusCode::ACCEPTED,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

#[axum::debug_handler]
pub async fn delete_user(
    State(state): State<AppState>,
    Json(delete_user_request): Json<DeleteUserRequest>,
) -> StatusCode {
    let response = state.database.delete_user(&delete_user_request).await;

    match response {
        Ok(user) => StatusCode::ACCEPTED,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
