use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug, Serialize, Deserialize)]
pub enum Error {
    #[error("Authentication failed")]
    Unauthorized,

    #[error("Access forbidden")]
    Forbidden,

    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Internal server error")]
    InternalError,

    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Cache error: {0}")]
    CacheError(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Rate limit exceeded")]
    RateLimitExceeded,
}

impl Error {
    pub fn status_code(&self) -> u16 {
        match self {
            Error::Unauthorized => 401,
            Error::Forbidden => 403,
            Error::NotFound(_) => 404,
            Error::BadRequest(_) => 400,
            Error::Conflict(_) => 409,
            Error::RateLimitExceeded => 429,
            _ => 500,
        }
    }
}
