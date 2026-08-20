use std::sync::Arc;
use crate::db::Database;

pub struct UserService {
    db: Arc<Database>,
}

impl UserService {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }
    
    // TODO: Implement user service methods
    // - create_user
    // - get_user_by_id
    // - get_user_by_email
    // - update_user
    // - verify_password
    // - send_friend_request
    // - accept_friend_request
}
