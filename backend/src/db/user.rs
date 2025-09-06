use deadpool_postgres::Pool;
use log::{debug, error};
// use crate::db::database::Database;

pub struct UserDB{
    pool: Pool,
    schema: String
}
impl UserDB{
    pub fn new(pool: Pool,schema: String)-> Self{
        UserDB{
            pool,
            schema
        }
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
        let client = match self.pool.get().await{
            Ok(c) => c,
            Err(e) => {
                error!("Error getting client from pool:{}",e);
                return;
            }
        };
        match client.execute(&statement, &[]).await{
            Ok(_) => debug!("User table created or already exists"),
            Err(e) => error!("Error creating user table: {}",e),
        }
    }
}