use std::sync::Arc;
use crate::db::{database::Database, user::UserDB};
use anyhow::Result;
use log::error;
pub mod database;
pub mod model;
pub mod user;

pub struct DBOrch{
    database: Option<Arc<Database>>,
    user: Option<UserDB>,
}

impl DBOrch{
    pub fn new() -> Self{
        DBOrch { database: None, user: None }
    }

    pub async fn init(&mut self) -> Result<(),>{
        let mut db = Database::new();
        db.init().await;
        if let Err(err) = db.create_tables().await{
            error!("{}",err);
        };
        self.database = Some(Arc::new(db));
        Ok(())
    }


    pub async fn init_instances(&mut self) -> Result<(),>{
        if self.database.is_none(){
            return Err(anyhow::anyhow!("Database not initialized"));
        }
        let db = self.database.as_ref().unwrap().clone();
        let userdb = UserDB::new(db,"sd".to_string());
        self.user = Some(userdb);
        Ok(())
    }
}