use crate::database::PoolType;
use crate::errors::ServiceError;
use actix_web::{Error, HttpRequest, HttpResponse, Responder};
use chrono;
use diesel::prelude::*;
use serde::Serialize;
use uuid::Uuid;

// use crate::{
//     config::db::Connection,
//     constants,
//     entity::user::token::UserToken,
//     schema::user_auth::{self, dsl::*},
// };

/// 登录信息表
#[derive(Debug, Serialize, Deserialize, Queryable)]
// #[table_name = "user_auth"]
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

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct AuthResponse {
    pub id: String,
    pub uid: String,
    pub identity_type: i32,
    pub identifier: String,
    pub certificate: String,
    pub login_session: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuthAccount {
    /// user id, the user-base id
    pub uid: String,
    pub phone: String,
}

impl UserAuth {
    /// find a userauth by the user's id or error out
    pub fn find(pool: &PoolType, uid: Uuid) -> Result<AuthResponse, ServiceError> {
        use crate::schema::user_auth::dsl::{id, user_auth};

        let not_found = format!("User {} not found", uid);
        let conn = pool.get()?;
        let auth = user_auth
            .filter(id.eq(uid.to_string()))
            .first::<AuthResponse>(&conn)
            .map_err(|_| ServiceError::NotFound(not_found))?;

        Ok(auth.into())
    }

    /// Find a user-auth by the user's authentication information
    pub fn find(
        pool: &PoolType,
        phone: &str,
        password: &str,
        account_type: &str,
    ) -> Result<AuthResponse, ServiceError> {
        use crate::schema::user_auth::dsl::{certificate, identifier, identity_type, user_auth};

        let conn = pool.get()?;
        let auth = user_auth
            .filter(identifier.eq(phone.to_string()))
            .filter(certificate.eq(password.to_string()))
            .filter(identity_type.eq(account_type))
            .first::<AuthResponse>(&conn)
            .map_err(|_| ServiceError::Unauthorized("Invalid login".into()))?;

        Ok(auth.into())
    }
}
