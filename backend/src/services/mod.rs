use std::sync::Arc;

use crate::configuration::Database;

pub mod auth_service;

pub struct Service{
    database: Arc<Database>,
}
