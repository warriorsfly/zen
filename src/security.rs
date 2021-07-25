use actix_web::{dev::ServiceRequest, Error, FromRequest};
use actix_web_httpauth::extractors::{
    bearer::{BearerAuth, Config},
    AuthenticationError,
};
use argon2rs::argon2i_simple;
use chrono::{Duration, Utc};
use futures::future::{err, ok, Ready};
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

impl FromRequest for Claims {
    type Error = ZenError;
    type Future = Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(
        _req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let _auth = _req.headers().get("Authorization");

        match _auth {
            Some(_) => {
                let _split: Vec<&str> = _auth.unwrap().to_str().unwrap().split("Bearer").collect();
                let token = _split[1].trim();
                match decode_jwt(token) {
                    Ok(claims) => ok(claims),
                    Err(e) => err(e),
                }
            }
            None => err(ZenError::Unauthorized(
                "no authorization in headers".to_string(),
            )),
        }
    }
}

pub(crate) fn create_jwt(claim: Claims) -> Result<String, ZenError> {
    let encoding_key = EncodingKey::from_secret(&CONFIG.jwt_key.as_ref());
    encode(&Header::default(), &claim, &encoding_key)
        .map_err(|e| ZenError::CannotEncodeTokenError(e.to_string()))
}

/// Decode a json web token (JWT)
pub(crate) fn decode_jwt(token: &str) -> Result<Claims, ZenError> {
    let decoding_key = DecodingKey::from_secret(&CONFIG.jwt_key.as_ref());
    decode::<Claims>(token, &decoding_key, &Validation::default())
        .map(|data| data.claims)
        .map_err(|e| ZenError::CannotDecodeTokenError(e.to_string()))
}

pub(crate) fn hash(password: &str) -> String {
    argon2i_simple(&password, &CONFIG.salt)
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect()
}

pub(crate) async fn bearer_validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, Error> {
    if let Ok(_claims) = decode_jwt(credentials.token()) {
        Ok(req)
    } else {
        let config = req
            .app_data::<Config>()
            .map(|data| data.clone())
            .unwrap_or_else(Default::default)
            .scope("urn:example:channel=HBO&urn:example:rating=G,PG-13");

        Err(AuthenticationError::from(config).into())
    }
}
