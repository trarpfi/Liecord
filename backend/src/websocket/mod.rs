use axum::{
    extract::{ws::{WebSocket, WebSocketUpgrade, Message}, State},
    response::Response,
};
use std::sync::Arc;
use futures::{sink::SinkExt, stream::StreamExt};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::AppState;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum WsMessage {
    // Authentication
    Authenticate { token: String },
    
    // Channel events
    ChannelJoin { channel_id: Uuid },
    ChannelLeave { channel_id: Uuid },
    
    // Message events
    MessageSend { channel_id: Uuid, content: String },
    MessageNew { message: serde_json::Value },
    MessageEdit { message_id: Uuid, content: String },
    MessageDelete { message_id: Uuid },
    
    // Typing indicators
    TypingStart { channel_id: Uuid },
    TypingStop { channel_id: Uuid },
    
    // Presence
    StatusChange { status: String },
    UserStatusUpdate { user_id: Uuid, status: String },
    
    // Voice/Video
    VoiceJoin { channel_id: Uuid },
    VoiceLeave { channel_id: Uuid },
    VoiceSignal { to: Uuid, signal: serde_json::Value },
    
    // Reactions
    ReactionAdd { message_id: Uuid, emoji: String },
    ReactionRemove { message_id: Uuid, emoji: String },
    
    // Error
    Error { message: String },
}

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
) -> Response {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: Arc<AppState>) {
    let (mut sender, mut receiver) = socket.split();
    
    tracing::info!("New WebSocket connection established");
    
    // TODO: Authenticate user from token
    // TODO: Join user-specific rooms
    // TODO: Set user online in Redis
    
    // Handle incoming messages
    while let Some(msg) = receiver.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                if let Ok(ws_msg) = serde_json::from_str::<WsMessage>(&text) {
                    match handle_ws_message(ws_msg, &state).await {
                        Ok(response) => {
                            if let Some(resp) = response {
                                let json = serde_json::to_string(&resp).unwrap();
                                if sender.send(Message::Text(json)).await.is_err() {
                                    break;
                                }
                            }
                        }
                        Err(e) => {
                            tracing::error!("Error handling WebSocket message: {}", e);
                            let error_msg = WsMessage::Error {
                                message: e.to_string(),
                            };
                            let json = serde_json::to_string(&error_msg).unwrap();
                            let _ = sender.send(Message::Text(json)).await;
                        }
                    }
                }
            }
            Ok(Message::Close(_)) => {
                tracing::info!("WebSocket connection closed");
                break;
            }
            Err(e) => {
                tracing::error!("WebSocket error: {}", e);
                break;
            }
            _ => {}
        }
    }
    
    // TODO: Set user offline in Redis
    // TODO: Broadcast user offline status
    tracing::info!("WebSocket connection terminated");
}

async fn handle_ws_message(
    msg: WsMessage,
    state: &Arc<AppState>,
) -> anyhow::Result<Option<WsMessage>> {
    match msg {
        WsMessage::ChannelJoin { channel_id } => {
            tracing::debug!("User joined channel: {}", channel_id);
            // TODO: Join channel room, broadcast presence
            Ok(None)
        }
        WsMessage::TypingStart { channel_id } => {
            tracing::debug!("User typing in channel: {}", channel_id);
            // TODO: Broadcast typing indicator
            Ok(None)
        }
        WsMessage::TypingStop { channel_id } => {
            tracing::debug!("User stopped typing in channel: {}", channel_id);
            // TODO: Broadcast typing stop
            Ok(None)
        }
        _ => {
            // TODO: Implement other message handlers
            Ok(None)
        }
    }
}
