use crate::database::PoolType;
use crate::errors::ServiceError;

use chrono;
use diesel::prelude::*;
use serde::Serialize;
use uuid::Uuid;

use crate::schema::user_auth::{self, dsl::*};

/// 登录信息表
#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable, Insertable)]
#[table_name = "user_auth"]
pub struct UserAuth {
    pub id: String,
    pub uid: String,
    pub identity_type: i32,
    pub identifier: String,
    pub certificate: String,
    pub login_session: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

/// remove password,remove created_at,updated_at
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct AuthResponse {
    pub id: Uuid,
    pub uid: String,
    pub identity_type: i32,
    pub identifier: String,
    pub login_session: String,
}

impl From<UserAuth> for AuthResponse {
    fn from(user: UserAuth) -> Self {
        AuthResponse {
            id: Uuid::parse_str(&user.id).unwrap(),
            uid: user.uid,
            identity_type: user.identity_type,
            identifier: user.identifier,
            login_session: user.login_session,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuthAccount {
    /// user id, the user-base id
    pub uid: String,
    pub phone: String,
}

/// find a userauth by the user's id or error out
pub fn find_by_id(pool: &PoolType, user_id: Uuid) -> Result<AuthResponse, ServiceError> {
    let not_found = format!("User {} not found", user_id);
    let conn = pool.get()?;
    let auth = user_auth
        .find(user_id.to_string())
        .first::<UserAuth>(&conn)
        .map_err(|_| ServiceError::NotFound(not_found))?;

    Ok(auth.into())
}

/// Find a user-auth by the user's authentication information
pub fn find_by_auth(
    pool: &PoolType,
    phone: &str,
    password: &str,
    account_type: i32,
) -> Result<AuthResponse, ServiceError> {
    let conn = pool.get()?;
    let auth = user_auth
        .filter(identifier.eq(phone.to_string()))
        .filter(certificate.eq(password.to_string()))
        .filter(identity_type.eq(account_type))
        .first::<UserAuth>(&conn)
        .map_err(|_| ServiceError::Unauthorized("Invalid login".into()))?;

    Ok(auth.into())
}
