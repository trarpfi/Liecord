use axum::{
    extract::{State, Path, Query},
    http::StatusCode,
    Json,
};
use std::sync::Arc;
use uuid::Uuid;
use serde::Deserialize;
use liecord_shared::models::{DirectMessage, Message};
use crate::AppState;

#[derive(Deserialize)]
pub struct CreateDmRequest {
    user_id: Uuid,
}

#[derive(Deserialize)]
pub struct GetMessagesQuery {
    limit: Option<i32>,
    before: Option<String>,
}

pub async fn create_or_get_dm(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateDmRequest>,
) -> Result<Json<DirectMessage>, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}

pub async fn get_user_dms(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<DirectMessage>>, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}

pub async fn get_dm_messages(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Query(params): Query<GetMessagesQuery>,
) -> Result<Json<Vec<Message>>, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}
