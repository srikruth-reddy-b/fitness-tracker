use std::sync::Arc;
use axum::{extract::State, response::IntoResponse, Json};
use serde::Deserialize;
use crate::{ services::auth_service::{AuthService}};

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub fullname: String,
    pub username: String,
    pub email: String,
    pub password: String,
    pub confirmpassword: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

// pub async fn register_handler(Json(payload): Json<User>) -> impl IntoResponse {
//     Json(serde_json::json!({
//         "message": format!("User {} registered successfully!", payload.username)
//     }));
//    let result = auth_service::AuthService::register(payload).await;
// }

pub struct Login{
    pub auth_service: Arc<AuthService>
}
impl Login{
    pub fn new(auth_service: Arc<AuthService>) -> Self{
        Login{
            auth_service
        }
    }
    pub async fn register_handler(
        State(auth_service): State<Arc<AuthService>>, 
        Json(payload): Json<RegisterRequest>
    ) -> impl IntoResponse {
        let result = auth_service.register(payload).await;
        Json(result)
    }
    pub async fn login_handler(
        State(auth_service): State<Arc<AuthService>>, 
        Json(payload): Json<LoginRequest>
    ) -> impl IntoResponse {
        let result = auth_service.login(payload).await;
        Json(result)
    }
}