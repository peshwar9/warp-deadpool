// Standard lib
use std::env;
// External crates - Primary
// External crates - Utilities
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
// Other internal modules
use crate::errors::MyError;

// Const and type declarations
// Struct declarations


#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub company: String,
    pub exp: usize,
}
// Functions

// Create a json web token (JWT)
pub async fn create_jwt() -> Result<String, MyError> {
    let key = env::var("JWT_SECRET").expect("JWT secret must be set");
    let env_sub = env::var("JWT_SUBJECT").expect("JWT secret must be set");
    let env_company = env::var("JWT_COMPANY").expect("JWT secret must be set");
    
    let my_claims = Claims {
        sub: env_sub.to_owned(),
        company: env_company.to_owned(),
        exp: 10000000000,
    };
    let encoding_key = EncodingKey::from_secret(key.as_bytes());

    encode(&Header::default(), &my_claims, &encoding_key)
        .map_err(|e| MyError::CannotEncodeJwtToken(e.to_string()))
    //  .map_err(|e| ResponseError(e.to_string()))
}

