use jsonwebtoken::{Header, encode, EncodingKey, errors::Error, Validation, decode, DecodingKey};
use log::error;
use std::env;

use crate::types::Claims;

pub fn generate_token(user_id: &str) -> Result<String, Error> {
    let header = Header::new(jsonwebtoken::Algorithm::HS256);
    let claims = Claims {
        sub: user_id.to_string(),
        exp: 10000000000
    };

    let token = env::var("TELEGRAM_TOKEN").expect("The telegram token is necessary");
    match encode(&header, &claims, &EncodingKey::from_secret(token.as_bytes())) {
        Ok(jwt) => return Ok(jwt),
        Err(err) => {
            error!("Failed to generate jwt: {err}");
            return Err(err);
        }
    }
}

pub fn validate_token(user_id: &str, jwt: &str) -> Result<(), Error> {
    let mut validation = Validation::new(jsonwebtoken::Algorithm::HS256);
    validation.sub = Some(user_id.to_string());

    let token = env::var("TELEGRAM_TOKEN").expect("The telegram token is necessary");

    if let Err(err) = decode::<Claims>(jwt, &DecodingKey::from_secret(token.as_bytes()), &validation) {
        error!("Invalid token: {err}");
        return Err(err);
    }

    Ok(())
}