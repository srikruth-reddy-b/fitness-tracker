use std::{sync::Arc, time};
use actix_web::{cookie::Cookie, web, HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, Header, EncodingKey};
use serde_json::json;
use ::time::Duration as TimeDuration;
use crate::services::{auth_service::{AuthResponse, AuthService}, jwt_service::JwtService};

// #[derive(Debug, Serialize, Deserialize)]
// pub struct Claims {
//     pub sub: String, 
//     iat: usize,    
//     exp: usize,    
// }

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    success: bool,
    message: String,
    token: String,
}

#[derive(Debug,Deserialize)]
pub struct RegisterRequest {
    pub fullname: String,
    pub username: String,
    pub email: String,
    pub password: String,
    pub confirmpassword: String,
}

#[derive(Debug,Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct ForgotPasswordRequest{
    pub username: String,
    pub password: String,
    pub confirmpassword: String
}

#[derive(Clone)]
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
        auth_service: web::Data<AuthService>,
        payload: web::Json<RegisterRequest>,
    ) -> impl Responder {
        let result = auth_service.register(payload.into_inner()).await;
        HttpResponse::Ok().json(result)
    }

    pub async fn login_handler(
        auth_service: web::Data<AuthService>,
        jwt_service: web::Data<JwtService>,
        payload: web::Json<LoginRequest>,
        ) -> impl Responder  {
        let result = auth_service.login(payload.into_inner()).await;
        if !result.success{
            return HttpResponse::Unauthorized().json(serde_json::json!({ "success": false, "message": format!("{}",result.message)}));
        }
        
        let cookie = Cookie::build("token", jwt_service.generate_token(
                &result.username).unwrap())
                .path("/")
                .max_age(TimeDuration::minutes(1))
                .same_site(actix_web::cookie::SameSite::None)
                .http_only(true)
                .finish();

        return HttpResponse::Ok()
                    .cookie(cookie)
                    .json(serde_json::json!({
                        "success": true,
                        "message": "Login successful"
                    }));
    }
    pub async fn forgot_password_handler(
        auth_service: web::Data<AuthService>,
        payload: web::Json<ForgotPasswordRequest>,
    ) -> impl Responder {
        let result = auth_service.forgot_password(payload.into_inner()).await;
        HttpResponse::Ok().json(result)
    }

    pub async fn verify_token_handler(
    jwt_service: web::Data<JwtService>,
    req: HttpRequest,
    ) -> impl Responder {
        // Try to extract cookie
        if let Some(cookie) = req.cookie("token") {
            let token = cookie.value();

            match jwt_service.validate_token(token) {
                Ok(claims) => {
                    HttpResponse::Ok().json(serde_json::json!({
                        "success": true,
                        "message": "Token is valid",
                        "username": claims.claims.sub
                    }))
                }
                Err(err) => {
                    HttpResponse::Unauthorized().json(serde_json::json!({
                        "success": false,
                        "message": format!("Invalid or expired token: {}", err)
                    }))
                }
            }
        } else {
            HttpResponse::Unauthorized().json(serde_json::json!({
                "success": false,
                "message": "No token cookie found"
            }))
        }
    }

}