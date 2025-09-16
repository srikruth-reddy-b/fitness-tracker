use axum::{response::IntoResponse, Json};
use serde::Deserialize;

use crate::services::auth_service;

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub fullname: String,
    pub username: String,
    pub email: String,
    pub password: String,
}

pub async fn register_handler(Json(payload): Json<RegisterRequest>) -> impl IntoResponse {
    Json(serde_json::json!({
        "message": format!("User {} registered successfully!", payload.username)
    }));
    let result = auth_service::AuthService::register(payload).await;
}