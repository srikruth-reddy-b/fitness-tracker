pub mod login;
pub mod register;
use axum::{Router, routing::post};

pub fn init() -> Router{
    Router::new().nest("/api", routes())
}

pub fn routes() -> Router{
    Router::new()
        .route("/login", post(login::login_handler))
        .route("/register",post(register::register_handler))
}