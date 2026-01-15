pub mod auth_service;
pub mod jwt_service;
pub mod post_service;
pub mod get_service;
pub mod put_service;

use std::sync::Arc;
use anyhow::{bail, Result};
use crate::{db::{database::DBOperations, logger::LoggerDB, user::UserDB, workouts::WorkoutDB}, 
            services::{auth_service::AuthService, get_service::GetService, jwt_service::JwtService, post_service::PostService, put_service::PutService}};

pub struct Service{
    pub auth_service: Option<Arc<AuthService>>,
    pub post_service: Option<Arc<PostService>>,
    pub get_service: Option<Arc<GetService>>,
    pub put_service: Option<Arc<PutService>>,
    pub jwt_service: Option<Arc<JwtService>>,
    pub database: Arc<DBOperations>,
}

impl Service{
    pub fn new(db_ops: Arc<DBOperations>)-> Self{
        Service { 
            auth_service: None,
            post_service: None,
            get_service: None,
            put_service: None,
            jwt_service: None,
            database: db_ops, 
        }
    }

    pub async fn init(&mut self) -> Result<(),> {
        let mut user_db = UserDB::new(self.database.clone());
        if let Err(err) = user_db.init().await{
            bail!("Error initialising user db: {}", err);
        };
        let user_arc = Arc::new(user_db);

        let mut workouts_db = WorkoutDB::new(self.database.clone());
        if let Err(err) = workouts_db.init().await{
            bail!("Error initialising workout db: {}", err);
        }
        let workout_db_arc = Arc::new(workouts_db);

        let mut logger_db = LoggerDB::new(self.database.clone());
        if let Err(err) = logger_db.init().await{
            bail!("Error initialising logger db: {}", err);
        }
        let logger_db_arc = Arc::new(logger_db);


        let auth_service = AuthService::new(user_arc.clone());
        self.auth_service = Some(Arc::new(auth_service));

        let jwt_service = JwtService::new();
        self.jwt_service = Some(Arc::new(jwt_service));
        
        let post_service = PostService::new(logger_db_arc.clone());
        self.post_service = Some(Arc::new(post_service));
        
        let get_service = GetService::new(workout_db_arc.clone(),user_arc.clone());
        self.get_service = Some(Arc::new(get_service));

        let put_service = PutService::new(logger_db_arc.clone());
        self.put_service = Some(Arc::new(put_service));
        Ok(())
    }
}