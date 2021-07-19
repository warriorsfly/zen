use actix_web::{Error, FromRequest};
use argon2rs::argon2i_simple;
use chrono::{Duration, Utc};
use futures::future::Ready;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::{config::CONFIG, errors::ZenError};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Claims {
    pub id: i32,
    pub email: String,
    expire_at: i64,
}

impl Claims {
    pub fn new(id: i32, email: String) -> Self {
        Self {
            id,
            email,
            expire_at: (Utc::now() + Duration::hours(CONFIG.jwt_expiration)).timestamp(),
        }
    }
}

// impl FromRequest for Claims {
//     type Error = Error;
//     type Future = Ready<Result<Self, Self::Error>>;
//     type Config = ();

//     fn from_request(req: &actix_web::HttpRequest, payload: &mut actix_web::dev::Payload) -> Self::Future {
//         todo!()
//     }
// }

pub(crate) fn create_jwt(claim: Claims) -> Result<String, ZenError> {
    let encoding_key = EncodingKey::from_secret(&CONFIG.jwt_key.as_ref());
    encode(&Header::default(), &claim, &encoding_key)
        .map_err(|e| ZenError::CannotEncodeTokenError(e.to_string()))
}

/// Decode a json web token (JWT)
pub fn decode_jwt(token: &str) -> Result<Claims, ZenError> {
    let decoding_key = DecodingKey::from_secret(&CONFIG.jwt_key.as_ref());
    decode::<Claims>(token, &decoding_key, &Validation::default())
        .map(|data| data.claims)
        .map_err(|e| ZenError::CannotDecodeTokenError(e.to_string()))
}

pub fn hash(password: &str) -> String {
    argon2i_simple(&password, &CONFIG.salt)
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect()
}
