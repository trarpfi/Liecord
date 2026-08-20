use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub display_name: String,
    pub avatar: Option<String>,
    pub banner: Option<String>,
    pub about: Option<String>,
    pub status: UserStatus,
    pub custom_status: Option<String>,
    pub badges: Vec<Badge>,
    pub nitro: Option<NitroSubscription>,
    pub friends: Vec<Uuid>,
    pub blocked_users: Vec<Uuid>,
    pub servers: Vec<Uuid>,
    pub created_at: DateTime<Utc>,
    pub last_seen: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum UserStatus {
    Online,
    Idle,
    Dnd,
    Offline,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Badge {
    Staff,
    Partner,
    VerifiedBot,
    EarlySupporter,
    Nitro,
    Nitro1Month,
    Nitro3Month,
    Nitro6Month,
    Nitro1Year,
    Nitro2Year,
    ServerBooster,
    HypesquadBravery,
    HypesquadBrilliance,
    HypesquadBalance,
    BugHunter,
    ActiveDeveloper,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NitroSubscription {
    pub active: bool,
    pub tier: NitroTier,
    pub start_date: DateTime<Utc>,
    pub expiry_date: DateTime<Utc>,
    pub subscription_months: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum NitroTier {
    Basic,
    Classic,
    Full,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub user: User,
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateProfileRequest {
    pub display_name: Option<String>,
    pub about: Option<String>,
    pub custom_status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FriendRequest {
    pub id: Uuid,
    pub from_user: Uuid,
    pub to_user: Uuid,
    pub status: FriendRequestStatus,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum FriendRequestStatus {
    Pending,
    Accepted,
    Rejected,
}
