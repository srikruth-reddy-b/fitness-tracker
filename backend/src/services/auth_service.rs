use std::sync::Arc;
use log::info;
use serde::Serialize;
use crate::{api::{login::LoginRequest}, db::{model::User, user::UserDB}};

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
    pub async fn login(request: LoginRequest) -> AuthResponse{
        println!("Authenticating user: {}", request.username);
        if request.username == "test" && request.password == "test"{
            println!("success");
            AuthResponse{
                success: true,
                message: "Login successful".to_string(),
            }
        }
        else{
            AuthResponse{
                success: false,
                message: "Invalid credentials".to_string(),
            }
        }
    }
    pub async fn register(&self, request: User) -> AuthResponse{
        // let user = User{
        //     fullname: request.fullname,
        //     username: request.username,
        //     email: request.email,
        //     password: request.password,
        // };
        match self.user.add_user(request).await{
            Ok(true) => {
                info!("User registered successfully");
                return AuthResponse{
                    success: true,
                    message: "User registered".to_string()
                }
            }
            Ok(false) => {
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