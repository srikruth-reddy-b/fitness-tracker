use actix_web::{cookie::Cookie, web, HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use ::time::Duration as TimeDuration;
use crate::{api::middleware::AuthenticatedUser,services::{auth_service::{AuthService}, get_service::GetService, jwt_service::JwtService}};

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
    pub weight: f64,
    pub height: f64,
    pub dob: String,
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

#[derive(Deserialize)]
pub struct UpdateUserInfo{
    pub fullname: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub weight: Option<f64>,
    pub height: Option<f64>,
    pub dob: Option<String>,
}

#[derive(Clone)]
pub struct Login{}
impl Login{
    pub fn new() -> Self{
        Login{}
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
        
        let user_id = result.user_id.unwrap_or_default(); // Should handle error ideally, but success implied user_id present
        let cookie = Cookie::build("token", jwt_service.generate_token(
                &result.username, user_id).unwrap())
                .path("/")
                .max_age(TimeDuration::minutes(60))
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
                        "username": claims.claims.sub,
                        "user_id": claims.claims.id
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

    pub async fn update_user_handler(auth_service: web::Data<AuthService>,
        user: AuthenticatedUser,
        payload: web::Json<UpdateUserInfo>,
    ) -> impl Responder {
        let id = user.id;
        let username = user.username;
        let update_info = payload.into_inner();
        let result = auth_service.update_user_details(id, username, update_info).await;
        HttpResponse::Ok().json(result)
    }

    pub async fn user_info_handler(get_service: web::Data<GetService>,
        user: AuthenticatedUser
    ) -> impl Responder{
        match get_service.get_user_info(user.id).await{
            Ok(user_info) => HttpResponse::Ok().json(user_info),
            Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
        }
    }
}