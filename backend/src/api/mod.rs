pub mod login_page;
use std::sync::Arc;
use log::error;
use actix_web::{web, App};
use crate::{api::login_page::Login, services::{auth_service::AuthService, jwt_service::JwtService}};

#[derive(Clone)]
pub struct API{
    auth_service: Arc<AuthService>,
    jwt_service: Arc<JwtService>,
    register_api : Option<Login>
}
impl API{
    pub fn new(service: Arc<AuthService>, jwt_service: Arc<JwtService>) -> Self{
        API{
            auth_service:service,
            jwt_service,
            register_api: None,
        }
    }
    pub async  fn init(&mut self ){
        let register_api = Login::new(self.auth_service.clone());
        self.register_api = Some(register_api);
        if self.register_api.is_none(){
            error!("Register API could not be initialised");
        }
    }
    pub fn configure(&self, cfg: &mut web::ServiceConfig){
        cfg.app_data(web::Data::from(self.auth_service.clone()))
           .app_data(web::Data::from(self.jwt_service.clone()));

        // configure routes
        cfg.service(
            web::scope("/api")
                .route("/login", web::post().to(crate::api::login_page::Login::login_handler))
                .route("/register", web::post().to(crate::api::login_page::Login::register_handler))
                .route("/forgot-password", web::post().to(crate::api::login_page::Login::forgot_password_handler))
                .route("/verify-token", web::get().to(crate::api::login_page::Login::verify_token_handler))
        );
    }
}