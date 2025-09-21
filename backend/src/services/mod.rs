pub mod auth_service;
pub mod jwt_service;

use std::sync::Arc;
use anyhow::Result;
use crate::{db::{database::DBOperations, user::UserDB}, services::auth_service::AuthService};

pub struct Service{
    pub auth_service: Option<Arc<AuthService>>,
    pub database: Arc<DBOperations>,
    pub user: Option<Arc<UserDB>>,
    pub schema: String,
}

impl Service{
    pub fn new(db_ops: Arc<DBOperations>,schema: String)-> Self{
        Service { 
            auth_service: None,
            database: db_ops, 
            user: None,
            schema,
        }
    }
    pub async fn init(&mut self) -> Result<(),> {
        let user_db = UserDB::new(self.database.clone(), self.schema.clone());
        self.user = Some(Arc::new(user_db));

        if self.user.is_none(){
            return Err(anyhow::anyhow!("User instance could not be created"));
        }
        let user = self.user.as_ref().unwrap().clone();
        let auth_service = AuthService::new(user);
        self.auth_service = Some(Arc::new(auth_service));
        Ok(())
    }
}