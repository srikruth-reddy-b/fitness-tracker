use std::sync::Arc;
use crate::db::{database::DBOperations, user::UserDB};
use anyhow::Result;
use log::error;
pub mod database;
pub mod model;
pub mod user;

pub struct Database{
    pub database: Option<Arc<DBOperations>>,
    user: Option<UserDB>,
    schema: String,
}

impl Database{
    pub fn new(schema: String) -> Self{
        Database { 
            database: None, 
            user: None,
            schema 
        }
    }

    pub async fn init(&mut self) -> Result<(),>{
        let mut db = DBOperations::new();
        if let Err(err) = db.init().await{
            error!("Error initialising database, {}",err)
        }
        // if let Err(err) = db.create_tables().await{
        //     error!("{}",err);
        // };
        self.database = Some(Arc::new(db));
        Ok(())
    }


    pub async fn init_instances(&mut self) -> Result<(),>{
        if self.database.is_none(){
            return Err(anyhow::anyhow!("Database not initialized"));
        }
        let db = self.database.as_ref().unwrap().clone();
        let mut userdb = UserDB::new(db,self.schema.clone());
        if let Err(err) = userdb.init().await{
            error!("Error initialising user db: {}",err);
        };
        self.user = Some(userdb);
        Ok(())
    }
}