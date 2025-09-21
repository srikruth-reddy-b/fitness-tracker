use std::sync::Arc;
use log::{error, info};
use serde::Serialize;
use crate::{api::login_page::{ForgotPasswordRequest, LoginRequest, RegisterRequest}, db::{model::User, user::UserDB}};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm, errors::Error as JwtError};
// use crate::api::login_page::Claims;
#[derive(Serialize)]
pub struct AuthResponse{
    pub username: String,
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
            Ok(true) =>{
                return AuthResponse{
                    username: request.username,
                    success: true,
                    message: "Login successful".to_string()
                }
            }
            Ok(false) => {
                return AuthResponse{
                    username: request.username,
                    success: false,
                    message: "Invalid credentials".to_string(),
                }
            }
            Err(err) =>{
                error!("Error during login: {}",err);
                return AuthResponse{
                    username: request.username,
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
                username: request.username,
                success: false,
                message: "Passwords do not match".to_string(),
            };
        }
        let user = User{
            fullname: request.fullname,
            username: request.username.clone(),
            email: request.email,
            password: request.password,
        };
        match self.user.add_user(user).await{
            Ok(true) => {
                info!("User registered successfully");
                return AuthResponse{
                    username: request.username,
                    success: true,
                    message: "User registered".to_string()
                }
            }
            Ok(false) => {
                info!("user exists");
                return AuthResponse{
                    username: request.username,
                    success: false,
                    message: "Username already exists".to_string(),
                }
            },
            Err(err) => {
                error!("Error during registration: {}",err);
                return AuthResponse{
                    username: request.username,
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
                success: false,
                message: "Passwords do not match".to_string(),
            };
        }
        match self.user.update_password(username, password).await{
            Ok(_) => {
                AuthResponse{
                    username: forgot_password.username,
                    success: true,
                    message: "Password updated".to_string()
                }
            }
            Err(err) => {
                error!("Error updating password: {}", err);
                return AuthResponse{
                    username: forgot_password.username,
                    success: false,
                    message: "Couldn't update password".to_string()
                }
            }
        }   
    }

    // pub async fn verify_jwt(&self, token: &str) -> Result<String, JwtError> {
    //     println!("called 2");
    //     let secret = "mysecretkey";
    //     let token_data = decode::<Claims>(
    //         token,
    //         &DecodingKey::from_secret(secret.as_ref()),
    //         &Validation::new(Algorithm::HS256),
    //     )?;
    //     println!("verfied jwt");
    //     println!("{:#?}",token_data);
    //     Ok(token_data.claims.sub)
    // }
}