use anyhow::Result;
use serde::{Deserialize,Serialize};
use std::io::Read;

const FILE_PATH: &str = "src/config.toml";
#[derive(Deserialize,Serialize,Default,Clone,Debug)]
pub struct Config{
    server: Server,pub 
    database: Database
}

#[derive(Deserialize,Serialize,Default,Clone,Debug)]
pub struct Server{
    pub ip : String,
    pub port: String,
}

#[derive(Deserialize,Serialize,Default,Clone,Debug)]
pub struct Database{
    pub username: String,
    pub password: String,
    pub host: String,
    pub port : u16,
    pub dbname: String,
    pub schema : String,
}

impl Config{
    pub fn load() -> Result<Self,>{
        let mut file = match std::fs::File::open(FILE_PATH){
            Ok(f) => f,
            Err(err) => {
                return Err(anyhow::anyhow!("Error opening file: {}",err));
            }
        };
        let mut contents = String::new();
        if let Err(err) = file.read_to_string(&mut contents){
            return Err(anyhow::anyhow!("Error writing contents to file: {}",err));
        };

        let config = match toml::from_str(&contents){
            Ok(t) => t,
            Err(err) => return Err(anyhow::anyhow!("Error retrieving config: {}",err))
        };
        Ok(config)
    }
    pub fn get_db_properties(&self) -> Database{
        Database { 
            username: self.database.username.clone(), 
            password: self.database.password.clone(),
            host: self.database.host.clone(), 
            port: self.database.port, 
            dbname: self.database.dbname.clone(), 
            schema: self.database.schema.clone(),
        }
    }
    pub fn get_server_properties(&self) -> Server{
        Server { 
            ip: self.server.ip.clone(), 
            port: self.server.port.clone(), 
        }
    }
    
}