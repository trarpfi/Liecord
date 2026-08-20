use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: Uuid,
    pub content: String,
    pub author_id: Uuid,
    pub channel_id: Option<Uuid>,
    pub dm_id: Option<Uuid>,
    pub attachments: Vec<Attachment>,
    pub embeds: Vec<Embed>,
    pub reactions: Vec<Reaction>,
    pub mentions: Vec<Uuid>,
    pub pinned: bool,
    pub edited: bool,
    pub edited_at: Option<DateTime<Utc>>,
    pub reply_to: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attachment {
    pub id: Uuid,
    pub filename: String,
    pub url: String,
    pub size: i64,
    pub content_type: String,
    pub width: Option<i32>,
    pub height: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Embed {
    pub title: Option<String>,
    pub description: Option<String>,
    pub url: Option<String>,
    pub color: Option<String>,
    pub thumbnail: Option<String>,
    pub image: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reaction {
    pub emoji: String,
    pub users: Vec<Uuid>,
    pub count: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateMessageRequest {
    pub content: String,
    pub reply_to: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateMessageRequest {
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageSearchRequest {
    pub query: String,
    pub channel_id: Option<Uuid>,
    pub author_id: Option<Uuid>,
    pub before: Option<DateTime<Utc>>,
    pub after: Option<DateTime<Utc>>,
    pub limit: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageSearchResponse {
    pub messages: Vec<Message>,
    pub total: i64,
}
