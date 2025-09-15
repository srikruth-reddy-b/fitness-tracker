use axum::{response::IntoResponse, Json};
use serde::Deserialize;

use crate::services::auth_service;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

pub async fn login_handler(Json(payload): Json<LoginRequest>) -> Json<serde_json::Value>{

    let result = auth_service::AuthService::login(payload).await;
    Json(serde_json::json!(result))
    //  if payload.username == "admin" && payload.password == "password" {
    //     Json(serde_json::json!({ "message": "Login successful" }))
    // } else {
    //     Json(serde_json::json!({ "error": "Invalid credentials" }))
    // }
}
