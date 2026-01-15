use std::sync::Arc;
use log::{debug, error, info};
use serde::Serialize;
use password_hash::{SaltString, rand_core::OsRng, PasswordHasher};
use crate::{api::login::{ForgotPasswordRequest, LoginRequest, RegisterRequest, UpdateUserInfo}, db::{model::{NewUser, UpdateUser}, user::{UserDB, ARGON}}};

#[derive(Serialize)]
pub struct AuthResponse{
    pub username: String,
    pub user_id: Option<i32>,
    pub success: bool,
    pub message: String,
}

pub struct AuthService{
    user: Arc<UserDB>,
}
impl AuthService{
    pub fn new(user: Arc<UserDB>) -> Self{
        AuthService { user }
    } 
    pub async fn login(&self,request: LoginRequest) -> AuthResponse{
        info!("Authenticating user: {}", request.username);
        let username = request.username.clone();
        let password = request.password;
        match self.user.verify_password(username, password).await{
            Ok(Some(id)) =>{
                info!("User {} authenticated successfully", request.username);
                return AuthResponse{
                    username: request.username,
                    user_id: Some(id),
                    success: true,
                    message: "Login successful".to_string()
                }
            }
            Ok(None) => {
                error!("No user found or Invalid credentials for user: {}", request.username);
                return AuthResponse{
                    username: request.username,
                    user_id: None,
                    success: false,
                    message: "Invalid credentials".to_string(),
                }
            }
            Err(err) =>{
                error!("Error during authentication for user {}: {}", request.username, err);
                return AuthResponse{
                    username: request.username,
                    user_id: None,
                    success: false,
                    message: format!("Error during authentication: {}",err),
                }
            }
        }
    }
    pub async fn register(&self, request: RegisterRequest) -> AuthResponse{
        if !request.password.eq(&request.confirmpassword){
            debug!("Passwords do not match for user: {}", request.username);
            return AuthResponse{
                username: request.username,
                user_id: None,
                success: false,
                message: "Passwords do not match".to_string(),
            };
        }
        let user = NewUser{
            fullname: &request.fullname,
            username: &request.username.clone(),
            email: &request.email,
            password: &request.password,
            weight: Some(request.weight),
            height: Some(request.height),
            dob: Some(request.dob.parse().unwrap_or_else(|_| chrono::NaiveDate::from_ymd_opt(1970,1,1).unwrap())),
        };
        match self.user.add_user(user).await{
            Ok(true) => {
                info!("User registered successfully");
                return AuthResponse{
                    username: request.username,
                    user_id: None,
                    success: true,
                    message: "User registered".to_string()
                }
            }
            Ok(false) => {
                info!("User already exists");
                return AuthResponse{
                    username: request.username,
                    user_id: None,
                    success: false,
                    message: "Username already exists".to_string(),
                }
            },
            Err(err) => {
                error!("Error during registration: {}",err);
                return AuthResponse{
                    username: request.username,
                    user_id: None,
                    success: false,
                    message: "Couldn't register user".to_string()
                }
            }
        }
    }

    pub async fn forgot_password(&self, forgot_password: ForgotPasswordRequest) -> AuthResponse{
        let username = forgot_password.username.clone();
        let password = forgot_password.password;
        let confirmpassword = forgot_password.confirmpassword;

        if !password.eq(&confirmpassword){
            return AuthResponse{
                username: forgot_password.username,
                user_id: None,
                success: false,
                message: "Passwords do not match".to_string(),
            };
        }
        match self.user.update_password(username, password).await{
            Ok(_) => {
                info!("Password updated successfully for user: {}", forgot_password.username);
                AuthResponse{
                    username: forgot_password.username,
                    user_id: None,
                    success: true,
                    message: "Password updated".to_string()
                }
            }
            Err(err) => {
                error!("Error updating password: {}", err);
                return AuthResponse{
                    username: forgot_password.username,
                    user_id: None,
                    success: false,
                    message: "Couldn't update password".to_string()
                }
            }
        }   
    }

    pub async fn update_user_details(&self, user_id: i32, username: String, user: UpdateUserInfo) -> AuthResponse{
        info!("Updating user details for user id: {}", user_id);
        let hashed_password = if let Some(pass) = &user.password {
            let salt = SaltString::generate(&mut OsRng);
            match ARGON.hash_password(pass.as_bytes(), &salt) {
                Ok(h) => Some(h.to_string()),
                Err(e) => {
                    error!("Error hashing password: {}", e);
                    return AuthResponse {
                        username: username,
                        user_id: Some(user_id),
                        success: false,
                        message: "Error updating password".to_string()
                    };
                }
            }
        } else {
            None
        };

        let userinfo = UpdateUser{
            fullname: user.fullname.as_deref(),
            email: user.email.as_deref(),
            password: hashed_password.as_deref(),
            weight: user.weight,
            height: user.height,
            dob: match user.dob{
                Some(dob_str) => Some(dob_str.parse().unwrap_or_else(|_| chrono::NaiveDate::from_ymd_opt(1970,1,1).unwrap())),
                None => None,
            },
        };
        match self.user.update_user_details(user_id, username.clone(), userinfo).await{
            Ok(_) => {
                info!("User details updated successfully for user id: {}", user_id);
                AuthResponse{
                    username,
                    user_id: Some(user_id),
                    success: true,
                    message: "User details updated".to_string()
                }
            }
            Err(err) => {
                error!("Error updating user details: {}", err);
                return AuthResponse{
                    username,
                    user_id: Some(user_id),
                    success: false,
                    message: "Couldn't update user details".to_string()
                }
            }
        }
    }
}