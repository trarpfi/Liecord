use axum::{
    extract::{State, Path},
    http::StatusCode,
    Json,
};
use std::sync::Arc;
use uuid::Uuid;
use liecord_shared::models::{User, UpdateProfileRequest};
use crate::AppState;

pub async fn get_user(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<User>, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}

pub async fn update_profile(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<UpdateProfileRequest>,
) -> Result<Json<User>, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}

pub async fn upload_avatar(
    State(state): State<Arc<AppState>>,
) -> Result<StatusCode, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}

pub async fn upload_banner(
    State(state): State<Arc<AppState>>,
) -> Result<StatusCode, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}

pub async fn update_status(
    State(state): State<Arc<AppState>>,
) -> Result<StatusCode, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}

pub async fn send_friend_request(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}

pub async fn accept_friend_request(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}

pub async fn remove_friend(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}
