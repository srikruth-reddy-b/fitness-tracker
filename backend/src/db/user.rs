use std::sync::Arc;
use anyhow::Result;
use deadpool_postgres::Pool;
use log::{debug, error, warn};
use password_hash::PasswordHasher;
use password_hash::{SaltString, rand_core::OsRng};
use tokio_postgres::{types::ToSql, Row};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use once_cell::sync::Lazy;
use crate::db::{database::Database, model::User};

pub static ARGON: Lazy<Argon2> = Lazy::new(|| Argon2::default());
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

    pub async fn add_user(&self, user: User) -> Result<bool,>{
        let username = user.username;
        let email = user.email;
        let fullname = user.fullname;
        let password = user.password;

        match self.search_username(username.clone()).await{
            Ok(true) => {
                warn!("Username already exists");
                return Ok(false);
            },
            Ok(false) => (),
            Err(err) => {
                return Err(anyhow::anyhow!("Error searching username: {}", err));
            }
        };
        
        let salt = SaltString::generate(&mut OsRng);
        let hashed = match ARGON.hash_password(password.as_bytes(), &salt){
            Ok(h) => h,
            Err(e) => return Err(anyhow::anyhow!("Error hashing password: {}",e)),
        }.to_string();
    
        let statement = format!("INSERT INTO {}.users (username, fullname, email, password) VALUES ($1, $2, $3, $4)",self.schema);
        let params: &[&(dyn ToSql + Sync)] = &[&username, &fullname, &email, &hashed];
        match self.database.execute(statement, params).await{
            Ok(_) => debug!("Registered user {}",username),
            Err(err) => return Err(anyhow::anyhow!("Error registering user: {}",err)),
        }
        Ok(true)
    }

    pub async fn search_username(&self, username: String) -> Result<bool,>{
        let statement = format!("SELECT 1 FROM {}.users WHERE username = $1 LIMIT 1;", self.schema);
        let params: &[&(dyn ToSql + Sync)] = &[&username];
        match self.database.query_opt(statement, params).await{
            Ok(Some(_)) => return Ok(true),
            Ok(None) => return Ok(false),
            Err(e) => return Err(anyhow::anyhow!("Error searching username: {}",e)),
        }    
    }
    
    pub async fn verify_password(&self, username:String, password: String) -> bool{
        let pass = match self.get_password_by_username(username.clone()).await{
            Ok(Some(p)) => p,
            Ok(None) => {
                warn!("No password found for username, {}",username);
                return false;
            }
            Err(err) => {
                error!("Error getting password by username: {}",err);
                return false;
            }
        };
        let parsed_hash = match PasswordHash::new(&pass) {
            Ok(h) => h,
            Err(_) => return false,
        };

        ARGON
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok()
    }

    pub async fn get_password_by_username(&self, username:String) -> Result<Option<String>,>{
        let statement = format!(
            "SELECT password FROM {}.users WHERE username = $1;",
            self.schema
        );
        let params: &[&(dyn ToSql + Sync)] = &[&username];
        match self.database.query_opt(statement, params).await{
            Ok(Some(row)) => {
                Ok(row.try_get("password").ok())
            }
            Ok(None) => Ok(None),
            Err(err) =>{
                return Err(anyhow::anyhow!("{}",err))
            }
        }
    }
}