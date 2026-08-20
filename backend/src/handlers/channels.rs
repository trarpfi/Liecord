use axum::{
    extract::{State, Path, Query},
    http::StatusCode,
    Json,
};
use std::sync::Arc;
use uuid::Uuid;
use serde::Deserialize;
use liecord_shared::models::{Channel, CreateChannelRequest, UpdateChannelRequest, Message};
use crate::AppState;

#[derive(Deserialize)]
pub struct GetMessagesQuery {
    limit: Option<i32>,
    before: Option<String>,
}

pub async fn create_channel(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateChannelRequest>,
) -> Result<(StatusCode, Json<Channel>), StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}

pub async fn get_channel(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<Channel>, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}

pub async fn update_channel(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateChannelRequest>,
) -> Result<Json<Channel>, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}

pub async fn delete_channel(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}

pub async fn get_messages(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Query(params): Query<GetMessagesQuery>,
) -> Result<Json<Vec<Message>>, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}
