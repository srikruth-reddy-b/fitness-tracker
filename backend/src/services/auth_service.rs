use std::sync::Arc;
use log::{error, info};
use serde::Serialize;
use crate::{api::login_page::{ForgotPasswordRequest, LoginRequest, RegisterRequest}, db::{model::User, user::UserDB}};

#[derive(Serialize)]
pub struct AuthResponse{
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
        let username = request.username;
        let password = request.password;
        match self.user.verify_password(username, password).await{
            Ok(true) =>{
                return AuthResponse{
                    success: true,
                    message: "Login successful".to_string()
                }
            }
            Ok(false) => {
                return AuthResponse{
                    success: false,
                    message: "Invalid credentials".to_string(),
                }
            }
            Err(err) =>{
                error!("Error during login: {}",err);
                return AuthResponse{
                    success: false,
                    message: format!("Error during authentication: {}",err),
                }
            }
        }
        // if request.username == "test" && request.password == "test"{
        //     println!("success");
        //     AuthResponse{
        //         success: true,
        //         message: "Login successful".to_string(),
        //     }
        // }
        // else{
            // AuthResponse{
            //     success: false,
            //     message: "Invalid credentials".to_string(),
            // }
        // }
    }
    pub async fn register(&self, request: RegisterRequest) -> AuthResponse{
        // let user = User{
            //     fullname: request.fullname,
        //     username: request.username,
        //     email: request.email,
        //     password: request.password,
        // };
        if !request.password.eq(&request.confirmpassword){
            return AuthResponse{
                success: false,
                message: "Passwords do not match".to_string(),
            };
        }
        let user = User{
            fullname: request.fullname,
            username: request.username,
            email: request.email,
            password: request.password,
        };
        match self.user.add_user(user).await{
            Ok(true) => {
                info!("User registered successfully");
                return AuthResponse{
                    success: true,
                    message: "User registered".to_string()
                }
            }
            Ok(false) => {
                info!("user exists");
                return AuthResponse{
                    success: false,
                    message: "Username already exists".to_string(),
                }
            },
            Err(err) => {
                error!("Error during registration: {}",err);
                return AuthResponse{
                    success: false,
                    message: "Couldn't register user".to_string()
                }
            }
        }
    }

    pub async fn forgot_password(&self, forgot_password: ForgotPasswordRequest) -> AuthResponse{
        let username = forgot_password.username;
        let password = forgot_password.password;
        let confirmpassword = forgot_password.confirmpassword;

        if !password.eq(&confirmpassword){
            return AuthResponse{
                success: false,
                message: "Passwords do not match".to_string(),
            };
        }
        match self.user.update_password(username, password).await{
            Ok(_) => {
                AuthResponse{
                    success: true,
                    message: "Password updated".to_string()
                }
            }
            Err(err) => {
                error!("Error updating password: {}", err);
                return AuthResponse{
                    success: false,
                    message: "Couldn't update password".to_string()
                }
            }
        }
        
    }
}