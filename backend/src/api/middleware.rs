use std::future::{ready, Ready};
use actix_web::{dev::Payload, web, FromRequest, HttpRequest, error::ErrorUnauthorized, Error};
use crate::services::jwt_service::JwtService;

#[derive(Debug)]
pub struct AuthenticatedUser {
    pub id: i32,
    pub username: String,
}

impl FromRequest for AuthenticatedUser {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let jwt_service = req.app_data::<web::Data<JwtService>>()
            .expect("JwtService not found in app data");

        if let Some(cookie) = req.cookie("token") {
             match jwt_service.validate_token(cookie.value()) {
                Ok(token_data) => {
                    return ready(Ok(AuthenticatedUser {
                        id: token_data.claims.id,
                        username: token_data.claims.sub,
                    }));
                }
                Err(_) => return ready(Err(ErrorUnauthorized("Invalid token"))),
            }
        }

        ready(Err(ErrorUnauthorized("No auth token found")))
    }
}
