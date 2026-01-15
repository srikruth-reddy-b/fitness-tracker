use std::sync::Arc;
use anyhow::bail;
use anyhow::Result;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use diesel_async::AsyncPgConnection;
use log::{debug, info, warn};
use password_hash::PasswordHasher;
use password_hash::{SaltString, rand_core::OsRng};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use once_cell::sync::Lazy;
use crate::db::model::NewUser;
use crate::db::model::UpdateUser;
use crate::db::{database::DBOperations, model::User};
use crate::schema::fittrack::users;
use diesel_async::pooled_connection::deadpool::Pool;
pub static ARGON: Lazy<Argon2> = Lazy::new(|| Argon2::default());

pub struct UserDB {
    database: Arc<DBOperations>,
    pool: Option<Pool<AsyncPgConnection>>
}

impl UserDB {
    pub fn new(database: Arc<DBOperations>) -> Self {
        UserDB { database, pool: None}
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
    }

    pub async fn add_user<'a>(&self, mut user: NewUser<'a>) -> Result<bool,>{
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
        
        user.password = &hashed;
        let inserted_user: User = match diesel::insert_into(users::table)
        .values(&user)
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
    }
    
    pub async fn verify_password(&self, username:String, password: String) -> Result<Option<i32>>{
        let (id, pass) = match self.get_id_and_password_by_username(username.clone()).await{
            Ok(Some(p)) => p,
            Ok(None) => {
                warn!("No password found for username, {}",username);
                return Ok(None);
            }
            Err(err) => {
                return Err(anyhow::anyhow!("Error getting password by username: {}",err));
            }
        };
        let parsed_hash = match PasswordHash::new(&pass) {
            Ok(h) => h,
            Err(err) => return Err(anyhow::anyhow!("{}",err)),
        };

        match ARGON.verify_password(password.as_bytes(), &parsed_hash).is_ok() {
            true => Ok(Some(id)),
            false => Ok(None),
        }
    }

    pub async fn get_id_and_password_by_username(&self, username:String) -> Result<Option<(i32, String)>>{
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
                .select((users::id, users::password))
                .first::<(i32, String)>(&mut conn)
                .await;

        match res {
            Ok(res) => Ok(Some(res)), 
            Err(diesel::result::Error::NotFound) => Ok(None), 
            Err(err) => return Err(anyhow::anyhow!("{}",err)),
        }
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
    }

    pub async fn update_user_details<'a>(&self, user_id: i32, username: String, user: UpdateUser<'a>) -> Result<(),>{
        let pool = match &self.pool{
            Some(pok) => pok,
            None => {
                bail!("Pool is not intialised");
            }
        };
        let mut conn = match pool.get().await{
            Ok(cok) => cok,
            Err(err) => {
                bail!("{}",err);
            }
        };

        match diesel::update(users::table)
            .filter(users::id.eq(user_id))
            .filter(users::username.eq(username))
            .set(user)
            .execute(&mut conn)
            .await{
                Ok(_) => debug!("Updated user details for user id {}", user_id),
                Err(err) => bail!("{}",err)
            };

        Ok(())
    }

    pub async fn get_user_by_id(&self, user_id: i32) -> Result<Option<User>>{
        let pool = match &self.pool{
            Some(pok) => pok,
            None => {
                bail!("Pool is not intialised");
            }
        };
        let mut conn = match pool.get().await{
            Ok(cok) => cok,
            Err(err) => {
                bail!("{}",err);
            }
        };

        let res = users::table
                .filter(users::id.eq(user_id))
                .first::<User>(&mut conn)
                .await;

        match res {
            Ok(user) => Ok(Some(user)), 
            Err(diesel::result::Error::NotFound) => Ok(None), 
            Err(err) => bail!("{}",err),
        }
    }
}