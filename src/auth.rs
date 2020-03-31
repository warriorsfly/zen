use crate::config::CONFIG;
use crate::errors::ServiceError;
use argon2rs::argon2i_simple;
use chrono::{Duration, Utc};
use derive_more::Display;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use uuid::Uuid;

#[derive(Clone, Debug, Display, Serialize, Deserialize, PartialEq)]
pub enum Claim {
    /// 手机登录
    #[display(fmt = "")]
    Phone { uid: Uuid, phone: String },
    /// 邮箱登录
    #[display(fmt = "")]
    Email { uid: Uuid, email: String },
    /// 邮箱登录
    #[display(fmt = "")]
    Name { uid: Uuid, name: String },
    /// QQ登录
    #[display(fmt = "")]
    QQ { uid: Uuid, qq: String },
    /// 邮箱登录
    #[display(fmt = "")]
    Wechat { uid: Uuid, openid: String },
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct PrivateClaim {
    pub uid: Uuid,
    pub identifier: String,
    pub identity_type: i32,
    exp: i64,
}

impl PrivateClaim {
    pub fn new(uid: Uuid, identifier: String, identity_type: i32) -> Self {
        Self {
            uid,
            identifier,
            identity_type,
            exp: (Utc::now() + Duration::hours(CONFIG.jwt_expiration)).timestamp(),
        }
    }
}

/// Create a json web token (JWT)
pub fn create_jwt(private_claim: PrivateClaim) -> Result<String, ServiceError> {
    let encoding_key = EncodingKey::from_secret(&CONFIG.jwt_key.as_ref());
    encode(&Header::default(), &private_claim, &encoding_key)
        .map_err(|e| ServiceError::EncodeTokenError(e.to_string()))
}

/// Decode a json web token (JWT)
pub fn decode_jwt(token: &str) -> Result<PrivateClaim, ServiceError> {
    let decoding_key = DecodingKey::from_secret(&CONFIG.jwt_key.as_ref());
    decode::<PrivateClaim>(token, &decoding_key, &Validation::default())
        .map(|data| data.claims)
        .map_err(|e| ServiceError::DecodeTokenError(e.to_string()))
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

/// Gets the identity service for injection into an Actix app
// pub fn get_identity_service() -> IdentityService<CookieIdentityPolicy> {
//     IdentityService::new(
//         CookieIdentityPolicy::new(&CONFIG.session_key.as_ref())
//             .name(&CONFIG.session_name)
//             .max_age_time(chrono::Duration::minutes(CONFIG.session_timeout))
//             .secure(CONFIG.session_secure),
//     )
// }

#[cfg(test)]
pub mod tests {
    use super::*;
    static PHONE: &str = "18326069658";

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
        // println!(format!(
        //     "hashed is {:02x},hashed_again is {:02x}",
        //     hashed,
        //     hashed_again.clone()
        // ));
        assert_eq!(hashed, hashed_again);
    }

    #[test]
    fn it_creates_a_jwt() {
        let private_claim = PrivateClaim::new(Uuid::new_v4(), PHONE.into(), 1);
        let jwt = create_jwt(private_claim);
        assert!(jwt.is_ok());
    }

    #[test]
    fn it_decodes_a_jwt() {
        let private_claim = PrivateClaim::new(Uuid::new_v4(), PHONE.into(), 1);
        let jwt = create_jwt(private_claim.clone()).unwrap();
        let decoded = decode_jwt(&jwt).unwrap();
        assert_eq!(private_claim, decoded);
    }
}
