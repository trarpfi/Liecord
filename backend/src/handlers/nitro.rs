use axum::{
    extract::{State, Path},
    http::StatusCode,
    Json,
};
use std::sync::Arc;
use uuid::Uuid;
use liecord_shared::models::{NitroPlans, SubscribeNitroRequest};
use serde_json::{json, Value};
use crate::AppState;

pub async fn subscribe(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<SubscribeNitroRequest>,
) -> Result<Json<Value>, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}

pub async fn cancel(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Value>, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}

pub async fn renew(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Value>, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}

pub async fn get_info() -> Json<NitroPlans> {
    Json(NitroPlans::default_plans())
}

pub async fn boost_server(
    State(state): State<Arc<AppState>>,
    Path(server_id): Path<Uuid>,
) -> Result<Json<Value>, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}
