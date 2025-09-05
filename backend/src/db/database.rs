use deadpool_postgres::{Manager, ManagerConfig, Pool, RecyclingMethod};
use log::error;
use tokio_postgres::NoTls;
use crate::configuration::Config;
use anyhow::Result;
pub struct Database{
    pool : Option<Pool>
}
impl Database{
    pub fn new() -> Self{
        Database { pool: None }
    }
    pub fn init(&mut self) {
        let mut pg_config = tokio_postgres::Config::new();
        let conf = match Config::load(){
            Ok(c) => c,
            Err(err) => {
                error!("{}",err);
                return;
            }
        };
        let database = conf.get_db_properties();
        
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
    pub async fn create_tables(&self){
        let pool = match self.get_pool().await{
            Ok(p) => p,
            Err(e) => {
                error!("Database not initalised");
                return;
            }
        };
    }
    
}