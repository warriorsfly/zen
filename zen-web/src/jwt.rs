use crate::config::CONFIG;
use crate::errors::ServError;
use argon2rs::argon2i_simple;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Claims {
    pub id: i32,
    pub exp: i64,
}

impl Claims {
    pub fn new(id: i32) -> Self {
        Self {
            id,
            exp: (Utc::now() + Duration::hours(CONFIG.jwt_expiration)).timestamp(),
        }
    }
}

/// Create a json web token (JWT)
pub fn create_jwt(claim: Claims) -> Result<String, ServError> {
    let encoding_key = EncodingKey::from_secret(&CONFIG.jwt_key.as_ref());
    encode(&Header::default(), &claim, &encoding_key)
        .map_err(|e| ServError::EncodeTokenError(e.to_string()))
}

/// Decode a json web token (JWT)
pub fn decode_jwt(token: &str) -> Result<Claims, ServError> {
    let decoding_key = DecodingKey::from_secret(&CONFIG.jwt_key.as_ref());
    decode::<Claims>(token, &decoding_key, &Validation::default())
        .map(|data| data.claims)
        .map_err(|e| ServError::DecodeTokenError(e.to_string()))
}

/// Encrypt a password
///
/// Uses the argon2i algorithm.
/// salt is environment-condigured.
pub fn hash(password: &str) -> String {
    argon2i_simple(&password, &CONFIG.auth_salt)
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect()
}

#[cfg(test)]
pub mod tests {
    use super::*;
    // static PHONE: &str = "18326069658";
    // static ID: Uuid = Uuid::new_v4();

    #[test]
    fn it_hashes_a_password() {
        let password = "password";
        let hashed = hash(password);
        assert_ne!(password, hashed);
    }

    #[test]
    fn it_matches_2_hashed_passwords() {
        let password = "password";
        let hashed = hash(password);
        let hashed_again = hash(password);
        println!("{}", hashed);
        println!("{}", hashed_again);
        assert_eq!(hashed, hashed_again);
    }

    #[test]
    fn it_creates_a_jwt() {
        let id = 1;
        let private_claim = Claims::new(id);
        let jwt = create_jwt(private_claim);
        assert!(jwt.is_ok());
    }

    #[test]
    fn it_decodes_a_jwt() {
        let id = 1;
        let private_claim = Claims::new(id);
        let jwt = create_jwt(private_claim.clone()).unwrap();
        let decoded = decode_jwt(&jwt).unwrap();
        assert_eq!(private_claim, decoded);
    }
}
