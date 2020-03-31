use crate::database::PoolType;
use crate::errors::ServiceError;

use chrono::{self, Utc};
use diesel::{insert_into, prelude::*};
use serde::Serialize;
use uuid::Uuid;

use super::base::UserBase;
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

/// 返回的[user_auth]实体,排除密码
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
pub struct AuthClaim {
    /// user id, the user-base id
    pub uid: String,
    pub identifier: String,
    pub identity_type: i32,
}

impl From<AuthResponse> for AuthClaim {
    fn from(user: AuthResponse) -> Self {
        AuthClaim {
            uid: user.uid,
            identity_type: user.identity_type,
            identifier: user.identifier,
        }
    }
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

///find by account and password
pub fn find_by_cert(
    pool: &PoolType,
    ident: &str,
    cert: &str,
    account_type: i32,
) -> Result<AuthResponse, ServiceError> {
    let conn = pool.get()?;

    match account_type {
        4 | 5 => Err(ServiceError::BadRequest("Invalid login".into())),
        _ => {
            let auth = user_auth
                .filter(identifier.eq(ident.to_string()))
                .filter(certificate.eq(cert.to_string()))
                .filter(identity_type.eq(account_type))
                .first::<UserAuth>(&conn)
                .map_err(|_| ServiceError::Unauthorized("Invalid login".into()))?;

            Ok(auth.into())
        }
    }
}
/// 根据第三方信息获取人员信息(微信)
pub fn find_by_3rd_account(
    pool: &PoolType,
    ident: &str,
    account_type: i32,
) -> Result<(AuthResponse, UserBase), ServiceError> {
    use crate::models::account::base::UserBaseDto;
    use crate::schema::user_base::{self, dsl::*};
    let conn = pool.get()?;

    match account_type {
        3 | 4 => {
            let auth = user_auth
                .filter(identifier.eq(ident.to_string()))
                .filter(identity_type.eq(account_type))
                .first::<UserAuth>(&conn)
                .map_err(|_| ServiceError::Unauthorized("Invalid login".into()));

            match auth {
                Ok(dto) => {
                    let base = user_base
                        .filter(id.eq(dto.uid.clone()))
                        .first::<UserBase>(&conn)
                        .map_err(|_| ServiceError::Unauthorized("Invalid login".into()))?;
                    Ok((dto.into(), base))
                }
                Err(_) => {
                    let user_id = Uuid::new_v4().to_string();
                    let dto = UserBaseDto {
                        id: &user_id,
                        user_role: 2,
                        register_source: 3,
                        nick_name: "WechatAccount", // diesel::insert_into(use_base)
                    };

                    insert_into(user_base)
                        .values(&dto)
                        .execute(&conn)
                        .map_err(|err| ServiceError::PoolError(err.to_string()));

                    let dto = UserAuth {
                        id: Uuid::new_v4().to_string(),
                        uid: user_id,
                        identity_type: 3,
                        identifier: ident.into(),
                        certificate: "".into(),
                        login_session: "".into(),
                        created_at: Utc::now().naive_utc(),
                        updated_at: Utc::now().naive_utc(),
                    };

                    insert_into(user_auth)
                        .values(&dto)
                        .execute(&conn)
                        .map_err(|err| ServiceError::PoolError(err.to_string()));

                    let base = user_base
                        .filter(id.eq(dto.uid.clone()))
                        .first::<UserBase>(&conn)
                        .map_err(|_| ServiceError::Unauthorized("Invalid login".into()))?;
                    Ok((dto.into(), base))
                }
            }
        }
        _ => Err(ServiceError::BadRequest(
            "Invalid login,illegal 3rd account".into(),
        )),
    }
}

#[cfg(test)]
pub mod tests {

    use super::*;
    use crate::tests::helpers::tests::get_pool;

    fn get_user_by_id(user_id: Uuid) -> Result<AuthResponse, ServiceError> {
        let pool = get_pool();
        find_by_id(&pool, user_id)
    }
}
