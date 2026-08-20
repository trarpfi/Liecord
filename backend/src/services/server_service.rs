use std::sync::Arc;
use crate::db::Database;

pub struct ServerService {
    db: Arc<Database>,
}

impl ServerService {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }
    
    // TODO: Implement server service methods
    // - create_server
    // - get_server
    // - update_server
    // - delete_server
    // - add_member
    // - remove_member
    // - create_invite
    // - join_with_invite
}
