use std::sync::Arc;
use anyhow::Result;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use diesel_async::AsyncPgConnection;
use log::{debug, info, warn};
use password_hash::PasswordHasher;
use password_hash::{SaltString, rand_core::OsRng};
// use tokio_postgres::{types::ToSql};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use once_cell::sync::Lazy;
use crate::db::model::NewUser;
use crate::db::{database::DBOperations, model::User};
use crate::schema::fittrack::users;
use diesel_async::pooled_connection::deadpool::Pool;
pub static ARGON: Lazy<Argon2> = Lazy::new(|| Argon2::default());
// use crate::db::database::Database;

pub struct UserDB {
    database: Arc<DBOperations>,
    schema: String,
    pool: Option<Pool<AsyncPgConnection>>
}

impl UserDB {
    pub fn new(database: Arc<DBOperations>,schema: String) -> Self {
        UserDB { database ,schema, pool: None}
    }

    pub async fn init(&mut self) -> Result<(),>{
        let pool = match self.database.get_pool().await{
            Ok(pool) => pool,
            Err(err) => {
                anyhow::bail!("{}",err);
            }
        };
        self.pool = Some(pool);
        Ok(())
        // let statement = format!("CREATE TABLE IF NOT EXISTS {}.users (
        //     id SERIAL ,
        //     username VARCHAR(50) PRIMARY KEY,
        //     fullname VARCHAR(100) NOT NULL,
        //     email VARCHAR(100) UNIQUE NOT NULL,
        //     password VARCHAR(255) NOT NULL,
        //     created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        // );",self.schema);

        // let params = &[];
        // match self.database.execute(statement, params).await{
        //     Ok(_) => debug!("User table created successfully"),
        //     Err(e) => error!("Error creating user table: {}",e),
        // }       
    }

    pub async fn add_user<'a>(&self, user: NewUser<'a>) -> Result<bool,>{
        let pool = match &self.pool{
            Some(pok) => pok,
            None => {
                anyhow::bail!("Pool is not initialised");
            }
        };
        let mut conn = match pool.get().await{
            Ok(cok) => cok,
            Err(err) => {
                anyhow::bail!("{}",err);
            }
        };
        let username = user.username;
        // let email = user.email;
        // let fullname = user.fullname;
        let password = user.password;

        match self.search_username(username.to_string().clone()).await{
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
        
        // let statement = format!("INSERT INTO {}.users (username, fullname, email, password) VALUES ($1, $2, $3, $4)",self.schema);
        // let params: &[&(dyn ToSql + Sync)] = &[&username, &fullname, &email, &hashed];
        // match self.database.execute(statement, params).await{
        //     Ok(_) => debug!("Registered user {}",username),
        //     Err(err) => return Err(anyhow::anyhow!("Error registering user: {}",err)),
        // }
        let new_user = NewUser{
            username : user.username,
            password: &hashed,
            email : user.email,
            fullname: user.fullname
        };
        let inserted_user: User = match diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(&mut conn)
        .await{
            Ok(iok) => iok,
            Err(err) => anyhow::bail!("{}",err)
        };
        debug!("Inserted user: {:?}", inserted_user);
        Ok(true)
    }

    pub async fn search_username(&self, username: String) -> Result<bool,>{
        let pool = match &self.pool{
            Some(pok) => pok,
            None => {
                anyhow::bail!("Pool is not initialised");
            }
        };
        let mut conn = match pool.get().await{
            Ok(cok) => cok,
            Err(err) => {
                anyhow::bail!("{}",err);
            }
        };

        let exists: std::result::Result<i32, diesel::result::Error> = users::table
                    .filter(users::username.eq(username))
                    .select(users::id)
                    .first::<i32>(&mut conn) 
                    .await;
        match exists {
        Ok(_) => Ok(true),
        Err(diesel::result::Error::NotFound) => Ok(false), 
        Err(e) => anyhow::bail!("{}",e), 
    }
        // let statement = format!("SELECT 1 FROM {}.users WHERE username = $1 LIMIT 1;", self.schema);
        // let params: &[&(dyn ToSql + Sync)] = &[&username];
        // match self.database.query_opt(statement, params).await{
        //     Ok(Some(_)) => return Ok(true),
        //     Ok(None) => return Ok(false),
        //     Err(e) => return Err(anyhow::anyhow!("Error searching username: {}",e)),
        // }    
    }
    
    pub async fn verify_password(&self, username:String, password: String) -> Result<bool,>{
        let pass = match self.get_password_by_username(username.clone()).await{
            Ok(Some(p)) => p,
            Ok(None) => {
                warn!("No password found for username, {}",username);
                return Ok(false);
            }
            Err(err) => {
                return Err(anyhow::anyhow!("Error getting password by username: {}",err));
            }
        };
        let parsed_hash = match PasswordHash::new(&pass) {
            Ok(h) => h,
            Err(err) => return Err(anyhow::anyhow!("{}",err)),
        };

        // match  ARGON.verify_password(password.as_bytes(), &parsed_hash){
        //     Ok(_) => Ok(true),
        //     Err(err) => return Err(anyhow::anyhow!("{}",err))
        // }
        let status = ARGON.verify_password(password.as_bytes(), &parsed_hash).is_ok();
        Ok(status)
    }

    pub async fn get_password_by_username(&self, username:String) -> Result<Option<String>,>{
        let pool = match &self.pool{
            Some(pok) => pok,
            None => {
                anyhow::bail!("Pool is not initialised");
            }
        };
        let mut conn = match pool.get().await{
            Ok(cok) => cok,
            Err(err) => {
                anyhow::bail!("{}",err);
            }
        };
        let res = users::table
                .filter(users::username.eq(username))
                .select(users::password)
                .first::<String>(&mut conn)
                .await;

        match res {
            Ok(pw) => Ok(Some(pw)), 
            Err(diesel::result::Error::NotFound) => Ok(None), 
            Err(err) => return Err(anyhow::anyhow!("{}",err)),
        }
        // let statement = format!(
        //     "SELECT password FROM {}.users WHERE username = $1;",
        //     self.schema
        // );
        // let params: &[&(dyn ToSql + Sync)] = &[&username];
        // match self.database.query_opt(statement, params).await{
        //     Ok(Some(row)) => {
        //         Ok(row.try_get("password").ok())
        //     }
        //     Ok(None) => Ok(None),
        //     Err(err) =>{
        //         return Err(anyhow::anyhow!("{}",err))
        //     }
        // }
    }

    pub async fn update_password(&self, username: String, password: String) -> Result<(),>{
        let pool = match &self.pool{
            Some(pok) => pok,
            None => {
                anyhow::bail!("Pool is not initialised");
            }
        };
        let mut conn = match pool.get().await{
            Ok(cok) => cok,
            Err(err) => {
                anyhow::bail!("{}",err);
            }
        };

        let salt = SaltString::generate(&mut OsRng);
        let hashed = match ARGON.hash_password(password.as_bytes(), &salt){
            Ok(h) => h,
            Err(e) => return Err(anyhow::anyhow!("Error hashing password: {}",e)),
        }.to_string();

        let username_clone = username.clone();
        match diesel::update(crate::db::user::users::dsl::users
            .filter(crate::db::user::users::dsl::username.eq(username)))
            .set(crate::db::user::users::dsl::password.eq(hashed))
            .execute(&mut conn)
            .await{
                Ok(uok) if uok > 0 => {
                    info!("Password updated for user {}", username_clone);
                    return Ok(())
                },
                Ok(_) => anyhow::bail!("Username not found"),
                Err(err) => anyhow::bail!("{}",err)
            };
        // let statement = format!(
        //     "UPDATE {}.users SET password = $1 WHERE username = $2",
        //     self.schema
        // );

        // let params: &[&(dyn ToSql + Sync)] = &[&hashed, &username];
        // match self.database.execute(statement, params).await {
        //     Ok(rows_updated) if rows_updated > 0 => {
        //         info!("Password updated for user {}", username);
        //         return Ok(())
        //     }
        //     Ok(_) => {
        //        return  Err(anyhow::anyhow!("Username not found"))
        //     }
        //     Err(err) => {
        //        return  Err(anyhow::anyhow!("Error updating password: {}", err))
        //     }
        // }
    }
}