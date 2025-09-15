use std::sync::Arc;
use anyhow::Result;
use deadpool_postgres::Pool;
use log::{debug, error};
use tokio_postgres::{types::ToSql, Row};

use crate::db::database::Database;
// use crate::db::database::Database;

pub struct UserDB {
    database: Database,
    schema: String,
}

impl<'a> UserDB {
    pub fn new(database: Database,schema: String) -> Self {
        UserDB { database ,schema}
    }

    pub async fn create_table(&self){
        let statement = format!("CREATE TABLE IF NOT EXISTS {}.users (
            id SERIAL ,
            username VARCHAR(50) PRIMARY KEY,
            fullname VARCHAR(100) NOT NULL,
            email VARCHAR(100) UNIQUE NOT NULL,
            password VARCHAR(255) NOT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        );",self.schema);

        let params = &[];
        match self.database.execute(statement, params).await{
            Ok(_) => debug!("User table created successfully"),
            Err(e) => error!("Error creating user table: {}",e),
        }       
    }

    pub async fn search_username(&self, username: String) -> Result<Row,>{
        let statement = format!("SELECT 1 FROM {}.users WHERE username = $1 LIMIT 1;", self.schema);
        let params: &[&(dyn ToSql + Sync)] = &[&username];
        let option_username = match self.database.query_opt(statement, params).await{
            Ok(c) => c,
            Err(e) => return Err(anyhow::anyhow!("Error searching username: {}",e)),
        };
        match option_username{
            Some(u) => {
                return Ok(u)
            },
            None => return Err(anyhow::anyhow!("Username not found")),
        } 
    }
    
}