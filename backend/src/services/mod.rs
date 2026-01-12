pub mod auth_service;
pub mod jwt_service;
pub mod post_service;
pub mod get_service;

use std::sync::Arc;
use anyhow::{bail, Result};
use crate::{db::{database::DBOperations, user::UserDB, workouts::WorkoutDB}, services::{auth_service::AuthService, post_service::PostService, get_service::GetService}};

pub struct Service{
    pub auth_service: Option<Arc<AuthService>>,
    pub post_service: Option<Arc<PostService>>,
    pub get_service: Option<Arc<GetService>>,
    pub database: Arc<DBOperations>,
    pub user: Option<Arc<UserDB>>,
    pub schema: String,
}

impl Service{
    pub fn new(db_ops: Arc<DBOperations>,schema: String)-> Self{
        Service { 
            auth_service: None,
            post_service: None,
            get_service: None,
            database: db_ops, 
            user: None,
            schema,
        }
    }
    pub async fn init(&mut self) -> Result<(),> {
        let mut user_db = UserDB::new(self.database.clone(), self.schema.clone());
        let _ = user_db.init().await;
        self.user = Some(Arc::new(user_db));

        if self.user.is_none(){
            return Err(anyhow::anyhow!("User instance could not be created"));
        }
        let user = self.user.as_ref().unwrap().clone();
        let auth_service = AuthService::new(user);
        self.auth_service = Some(Arc::new(auth_service));

        let mut workouts_db = WorkoutDB::new(self.database.clone());
        if let Err(err) = workouts_db.init().await{
            bail!("Error initialising workout db: {}", err);
        }
        let workout_db_arc = Arc::new(workouts_db);
        
        let post_service = PostService::new(workout_db_arc.clone());
        self.post_service = Some(Arc::new(post_service));
        
        let get_service = GetService::new(workout_db_arc.clone());
        self.get_service = Some(Arc::new(get_service));
        Ok(())
    }
}