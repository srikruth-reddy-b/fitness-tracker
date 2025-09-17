pub mod login_page;
use std::sync::Arc;

use axum::{Router, routing::post};
use log::error;

use crate::{api::login_page::Login, services::auth_service::AuthService};

// pub fn init() -> Router{
//     Router::new().nest("/api", routes())
// }

// pub fn routes() -> Router{
//     Router::new()
//         .route("/login", post(login::login_handler))
//         .route("/register",post(register::register_handler))
// }

pub struct API{
    auth_service: Arc<AuthService>,
    register_api : Option<Login>
}
impl API{
    pub fn new(service: Arc<AuthService>) -> Self{
        API{
            auth_service:service,
            register_api: None,
        }
    }
    pub async fn init(&mut self)-> Router{
        let register_api = Login::new(self.auth_service.clone());
        self.register_api = Some(register_api);
        if self.register_api.is_none(){
            error!("Register API could not be initialised");
        }
        Router::new().nest("/api", self.routes())
    }
    pub fn routes(&self) -> Router{
        // let register = self.register_api.unwrap();
        Router::new()
            .route("/login", post(Login::login_handler))
            .with_state(self.auth_service.clone())
            .route("/register", post(Login::register_handler))
            .with_state(self.auth_service.clone())
    }

}