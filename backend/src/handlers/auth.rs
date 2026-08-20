use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use std::sync::Arc;
use liecord_shared::models::{CreateUserRequest, LoginRequest, LoginResponse, User};
use crate::AppState;

pub async fn register(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<User>), StatusCode> {
    // TODO: Implement user registration with password hashing
    Err(StatusCode::NOT_IMPLEMENTED)
}

pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {
    // TODO: Implement login with JWT token generation
    Err(StatusCode::NOT_IMPLEMENTED)
}

pub async fn get_current_user(
    State(state): State<Arc<AppState>>,
) -> Result<Json<User>, StatusCode> {
    // TODO: Get current user from JWT token
    Err(StatusCode::NOT_IMPLEMENTED)
}
