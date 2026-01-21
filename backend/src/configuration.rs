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
        let mut contents = String::new();
        
        // Try to read the file, but don't fail if it's missing (Docker might use only env vars)
        if let Ok(mut file) = std::fs::File::open(FILE_PATH) {
             if let Err(err) = file.read_to_string(&mut contents){
                return Err(anyhow::anyhow!("Error writing contents to file: {}",err));
            };
        }

        let mut config: Config = if contents.is_empty() {
            Config::default()
        } else {
            match toml::from_str(&contents){
                Ok(t) => t,
                Err(err) => return Err(anyhow::anyhow!("Error retrieving config: {}",err))
            }
        };

        // Override with Environment Variables (Best practice for Docker)
        if let Ok(host) = std::env::var("DATABASE_HOST") { config.database.host = host; }
        if let Ok(port) = std::env::var("DATABASE_PORT") { 
            if let Ok(p) = port.parse::<u16>() { config.database.port = p; }
        }
        if let Ok(user) = std::env::var("DATABASE_USER") { config.database.username = user; }
        if let Ok(pass) = std::env::var("DATABASE_PASSWORD") { config.database.password = pass; }
        if let Ok(name) = std::env::var("DATABASE_NAME") { config.database.dbname = name; }
        if let Ok(ip) = std::env::var("SERVER_IP") { config.server.ip = ip; }
        if let Ok(port) = std::env::var("SERVER_PORT") { config.server.port = port; }
        
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