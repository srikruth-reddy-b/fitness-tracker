use std::sync::Arc;
use serde::Serialize;
use crate::{api::{login::LoginRequest, register::RegisterRequest}, configuration::Database};

#[derive(Serialize)]
pub struct AuthResponse{
    pub success: bool,
    pub message: String,
}

pub struct AuthService{
    database: Arc<Database>,
}
impl AuthService{
    pub fn new(database: Arc<Database>) -> Self{
        AuthService { database }
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
    pub async fn register(request: RegisterRequest) -> AuthResponse{
        
        AuthResponse{
                success: true,
                message: "Login successful".to_string(),
            }
    }
}