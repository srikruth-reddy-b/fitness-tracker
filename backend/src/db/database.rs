use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use log::error;
use crate::{configuration::Config};
use anyhow::Result;
use diesel_async::{AsyncPgConnection}; 
use diesel_async::pooled_connection::deadpool::Pool;

#[derive(Clone)]
pub struct DBOperations{
    pool: Option<Pool<AsyncPgConnection>>,
    schema : String,
}

impl DBOperations{
    pub fn new() -> Self{
        DBOperations { 
            pool: None,
            schema : String::new(),
        }
    }
    pub async fn init(&mut self) -> Result<String,>{

        let conf = match Config::load(){
            Ok(c) => c,
            Err(err) => {
                error!("{}",err);
                return Ok("".to_string());
            }
        };
        let database = conf.get_db_properties();
        let connection_url = format!("postgres://{}:{}@{}:{}/{}",
            database.username,  // Username
            database.password,     // Password
            database.host,      // Host
            database.port,      // Port
            database.dbname    // Database name 
        );
        let config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(connection_url);
        let pool = match Pool::builder(config).max_size(5).build(){
            Ok(pok) => pok,
            Err(err) => {
                anyhow::bail!("{}",err)
            }
        };
        self.pool = Some(pool);
        let schema = database.schema;
        self.schema = schema.clone();

        Ok(schema)
    }

    pub async fn get_pool(&self) -> Result<Pool<AsyncPgConnection>> {
        match &self.pool {
            Some(p) => Ok(p.clone()),
            None => Err(anyhow::anyhow!("Database not initialised")),
        }
    }
    
}