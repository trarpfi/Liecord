use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Server {
    pub id: Uuid,
    pub name: String,
    pub icon: Option<String>,
    pub banner: Option<String>,
    pub description: Option<String>,
    pub owner_id: Uuid,
    pub verification_level: VerificationLevel,
    pub default_notifications: NotificationLevel,
    pub boost_level: i32,
    pub boost_count: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerMember {
    pub server_id: Uuid,
    pub user_id: Uuid,
    pub nickname: Option<String>,
    pub roles: Vec<Uuid>,
    pub joined_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Role {
    pub id: Uuid,
    pub server_id: Uuid,
    pub name: String,
    pub color: String,
    pub position: i32,
    pub permissions: Permissions,
    pub hoist: bool,
    pub mentionable: bool,
    pub is_default: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Permissions {
    pub administrator: bool,
    pub manage_server: bool,
    pub manage_roles: bool,
    pub manage_channels: bool,
    pub kick_members: bool,
    pub ban_members: bool,
    pub create_invite: bool,
    pub change_nickname: bool,
    pub manage_nicknames: bool,
    pub manage_emojis: bool,
    pub view_channels: bool,
    pub send_messages: bool,
    pub send_tts_messages: bool,
    pub manage_messages: bool,
    pub embed_links: bool,
    pub attach_files: bool,
    pub read_message_history: bool,
    pub mention_everyone: bool,
    pub use_external_emojis: bool,
    pub add_reactions: bool,
    pub connect: bool,
    pub speak: bool,
    pub mute_members: bool,
    pub deafen_members: bool,
    pub move_members: bool,
    pub use_voice_activity: bool,
}

impl Default for Permissions {
    fn default() -> Self {
        Self {
            administrator: false,
            manage_server: false,
            manage_roles: false,
            manage_channels: false,
            kick_members: false,
            ban_members: false,
            create_invite: false,
            change_nickname: true,
            manage_nicknames: false,
            manage_emojis: false,
            view_channels: true,
            send_messages: true,
            send_tts_messages: false,
            manage_messages: false,
            embed_links: true,
            attach_files: true,
            read_message_history: true,
            mention_everyone: false,
            use_external_emojis: true,
            add_reactions: true,
            connect: true,
            speak: true,
            mute_members: false,
            deafen_members: false,
            move_members: false,
            use_voice_activity: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum VerificationLevel {
    None,
    Low,
    Medium,
    High,
    Highest,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum NotificationLevel {
    All,
    Mentions,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateServerRequest {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateServerRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub verification_level: Option<VerificationLevel>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Invite {
    pub code: String,
    pub server_id: Uuid,
    pub creator_id: Uuid,
    pub uses: i32,
    pub max_uses: i32,
    pub expires_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateInviteRequest {
    pub max_uses: Option<i32>,
    pub expires_in: Option<i64>, // seconds
}
