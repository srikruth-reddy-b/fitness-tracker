use chrono::{Utc, Duration};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey, TokenData, errors::Result as JwtResult};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,   
    pub exp: usize,   
    pub iat: usize,    
}

#[derive(Clone)]
pub struct JwtService {
    secret: String,
    expiration_minutes: i64,
}

impl JwtService {
    pub fn new() -> Self {
        JwtService {
            secret : "SECRET".to_string(),
            expiration_minutes: 1,
        }
    }

    pub fn generate_token(&self, subject: &str) -> JwtResult<String> {
        let now = Utc::now();
        let exp = now + Duration::minutes(self.expiration_minutes);

        let claims = Claims {
            sub: subject.to_owned(),
            iat: now.timestamp() as usize,
            exp: exp.timestamp() as usize,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
    }

    pub fn validate_token(&self, token: &str) -> JwtResult<TokenData<Claims>> {
        decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &Validation::default(),
        )
    }
}
