use axum::{
    extract::{State, Path},
    http::StatusCode,
    Json,
};
use std::sync::Arc;
use uuid::Uuid;
use liecord_shared::models::{Server, CreateServerRequest, UpdateServerRequest, Invite, CreateInviteRequest};
use crate::AppState;

pub async fn create_server(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateServerRequest>,
) -> Result<(StatusCode, Json<Server>), StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}

pub async fn get_server(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<Server>, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}

pub async fn get_user_servers(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<Server>>, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}

pub async fn update_server(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateServerRequest>,
) -> Result<Json<Server>, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}

pub async fn delete_server(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}

pub async fn upload_icon(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}

pub async fn create_invite(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(payload): Json<CreateInviteRequest>,
) -> Result<Json<Invite>, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}

pub async fn join_server(
    State(state): State<Arc<AppState>>,
    Path(code): Path<String>,
) -> Result<Json<Server>, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}

pub async fn leave_server(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}
