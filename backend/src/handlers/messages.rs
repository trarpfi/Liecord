use axum::{
    extract::{State, Path},
    http::StatusCode,
    Json,
};
use std::sync::Arc;
use uuid::Uuid;
use liecord_shared::models::{Message, CreateMessageRequest, UpdateMessageRequest, MessageSearchRequest, MessageSearchResponse};
use crate::AppState;

pub async fn send_message(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateMessageRequest>,
) -> Result<(StatusCode, Json<Message>), StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}

pub async fn edit_message(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateMessageRequest>,
) -> Result<Json<Message>, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}

pub async fn delete_message(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}

pub async fn add_reaction(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}

pub async fn remove_reaction(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}

pub async fn search_messages(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<MessageSearchRequest>,
) -> Result<Json<MessageSearchResponse>, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}
