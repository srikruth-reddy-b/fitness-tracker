use axum::{response::IntoResponse, Json};
use serde::Deserialize;

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
    }))
}