use deadpool_postgres::{Manager, ManagerConfig, Pool, RecyclingMethod};
use log::error;
use tokio_postgres::{NoTls, Row};
use crate::{configuration::Config, db::user::UserDB};
use anyhow::Result;

#[derive(Clone)]
pub struct DBOperations{
    pool : Option<Pool>,
    schema : String,
}

impl DBOperations{
    pub fn new() -> Self{
        DBOperations { 
            pool: None,
            schema : String::new(),
        }
    }
    pub async fn init(&mut self) -> String{
        let mut pg_config = tokio_postgres::Config::new();
        let conf = match Config::load(){
            Ok(c) => c,
            Err(err) => {
                error!("{}",err);
                return "".to_string();
            }
        };
        let database = conf.get_db_properties();
        let schema = database.schema;
        self.schema = schema.clone();

        pg_config.host(database.host);
        pg_config.user(database.username);
        pg_config.port(database.port);
        pg_config.password(database.password);
        pg_config.dbname(database.dbname);

        let mgr_config = ManagerConfig{
            recycling_method : RecyclingMethod::Fast
        };
        let mgr = Manager::from_config(pg_config, NoTls, mgr_config);
        
        let pool = Pool::builder(mgr).max_size(5).build().unwrap();
        self.pool = Some(pool);
        schema
        // self.init_instances().await;
    }

    pub async fn init_instances(&mut self) {
        let pool = match self.get_pool().await{
            Ok(p) => p,
            Err(e) => {
                error!("{}",e);
                return;
                // return Err(anyhow::anyhow!(e)); 
            }
        };
    }
    pub async fn get_pool(&self) -> Result<Pool,>{
        let pool = match &self.pool{
            Some(p) => p,
            None => {
                return Err(anyhow::anyhow!("Database not initalised"));
            }
        };
        Ok(pool.clone())
    }
    pub async fn create_tables(&self) -> Result<(),>{
        let pool = match self.get_pool().await{
            Ok(p) => p,
            Err(e) => {
                return Err(anyhow::anyhow!(e)); 
            }
        };
        let statement = format!("CREATE SCHEMA IF NOT EXISTS {};",self.schema);
        let client = match pool.get().await{
            Ok(c) => c,
            Err(err) => return Err(anyhow::anyhow!("Failed to get client from pool: {}", err)),
        };
        match client.execute(&statement, &[]).await {
            Ok(_) => {},
            Err(err) => return Err(anyhow::anyhow!("Failed to create {} schema: {}", self.schema, err)),
        };
        // if self.userdb.is_none(){
        //     return
        // }
        Ok(())
    }

    pub async fn execute(&self, statement: String, params: &[&(dyn tokio_postgres::types::ToSql + Sync)]) -> Result<u64,>{
        let pool = match &self.pool{
            Some(p) => p,
            None => {
                return Err(anyhow::anyhow!("Database not initalised"));
            }
        };
        let client = match pool.get().await{
            Ok(c) => c,
            Err(e) => {
                return Err(anyhow::anyhow!("Pool is closed: {}", e));
            }
        };
        match client.execute(&statement, params).await{
            Ok(r ) => return Ok(r),
            Err(err) => {
                return Err(anyhow::anyhow!("Failed to execute statement: {}",err))
            }
        }
    }

    pub async fn query(&self, statement: String, params: &[&(dyn tokio_postgres::types::ToSql + Sync)]) -> Result<Vec<Row>,>{
        let pool = match &self.pool{
            Some(p) => p,
            None => {
                return Err(anyhow::anyhow!("Database not initalised"));
            }
        };
        let client = match pool.get().await{
            Ok(c) => c,
            Err(e) => {
                return Err(anyhow::anyhow!("Pool is closed: {}", e));
            }
        };
         match client.query(&statement, params).await{
            Ok(rows ) => return Ok(rows),
            Err(err) => {
                return Err(anyhow::anyhow!("Failed to execute statement: {}",err))
            }
        }
    }

    pub async fn query_opt(&self, statement: String, params: &[&(dyn tokio_postgres::types::ToSql + Sync)]) -> Result<Option<Row>,>{
        let pool = match &self.pool{
            Some(p) => p,
            None => {
                return Err(anyhow::anyhow!("Database not initalised"));
            }
        };
        let client = match pool.get().await{
            Ok(c) => c,
            Err(e) => {
                return Err(anyhow::anyhow!("Pool is closed: {}", e));
            }
        };
         match client.query_opt(&statement, params).await{
            Ok(row ) => return Ok(row),
            Err(err) => {
                return Err(anyhow::anyhow!("Failed to execute statement: {}",err))
            }
        }
    } 
    
}