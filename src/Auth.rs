use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
    Json,
    RequestPartsExt,
};

use std::{time::{Duration, SystemTime}};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::env;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: i32,        
    pub exp: usize,      
    pub iat: usize,      
    pub iss: String,     
}

pub fn create_token(user_id: i32) -> Result<String, jsonwebtoken::errors::Error> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let now = time::UtcDateTime::now();
    let claims = Claims {
        sub: user_id,
        exp: (now + Duration::from_mins(15)).unix_timestamp() as usize, 
        iat: now.unix_timestamp() as usize,
        iss: "task_scheduler".to_string(),
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}

pub fn verify_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let mut validation = Validation::default();
    validation.set_issuer(&["task_scheduler"]);


    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &validation,
    )?;
    Ok(token_data.claims)
}


pub struct AuthUser(pub Claims);


