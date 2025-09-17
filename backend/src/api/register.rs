use std::sync::Arc;
use axum::{extract::State, response::IntoResponse, Json};
use serde::Deserialize;
use crate::{db::{model::User, user::UserDB}, services::auth_service::{self, AuthService}};

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub fullname: String,
    pub username: String,
    pub email: String,
    pub password: String,
}

// pub async fn register_handler(Json(payload): Json<User>) -> impl IntoResponse {
//     Json(serde_json::json!({
//         "message": format!("User {} registered successfully!", payload.username)
//     }));
//    let result = auth_service::AuthService::register(payload).await;
// }

pub struct Register{
    pub auth_service: Arc<AuthService>
}
impl Register{
    pub fn new(auth_service: Arc<AuthService>) -> Self{
        Register{
            auth_service
        }
    }
    // pub async fn register_handler(&self, Json(payload): Json<User>) -> impl IntoResponse {
    //     let _ = Json(serde_json::json!({
    //         "message": format!("User {} registered successfully!", payload.username)
    //     }));
    //     let result = self.auth_service.register(payload).await;
    // }
    pub async fn register_handler(
        State(auth_service): State<Arc<AuthService>>, 
        Json(payload): Json<User>
    ) -> impl IntoResponse {
        let result = auth_service.register(payload).await;

        Json(serde_json::json!({
            "message": format!("User registered successfully!")
        }))
    }
}