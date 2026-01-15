use std::sync::Arc;
use crate::db::{database::DBOperations};
use anyhow::Result;
use log::error;
pub mod database;
pub mod model;
pub mod user;
pub mod workouts;
pub mod logger;

pub struct Database{
    pub database: Option<Arc<DBOperations>>,
}

impl Database{
    pub fn new() -> Self{
        Database { 
            database: None, 
        }
    }

    pub async fn init(&mut self) -> Result<(),>{
        let mut db = DBOperations::new();
        if let Err(err) = db.init().await{
            error!("Error initialising database, {}",err)
        }
        self.database = Some(Arc::new(db));
        Ok(())
    }

}