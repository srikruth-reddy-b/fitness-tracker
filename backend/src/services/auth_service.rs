use std::sync::Arc;
use log::info;
use serde::Serialize;
use crate::{api::{ login_page::{RegisterRequest,LoginRequest}}, db::{model::User, user::UserDB}};

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
            Err(_err) => {
                return AuthResponse{
                    success: false,
                    message: "Couldnot register user".to_string()
                }
            }
        }
    }
}