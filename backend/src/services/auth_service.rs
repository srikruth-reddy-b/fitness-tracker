use serde::Serialize;
use crate::api::{login::LoginRequest, register::RegisterRequest};

#[derive(Serialize)]
pub struct AuthResponse{
    pub success: bool,
    pub message: String,
}

pub struct AuthService;
impl AuthService{
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