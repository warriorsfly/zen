use crate::database::PoolType;
use crate::errors::ServiceError;

use chrono::{self, Utc};
use diesel::{insert_into, prelude::*};
use serde::Serialize;
use uuid::Uuid;

use super::base::UserBase;
use crate::schema::user_auth::{self, dsl::*};

/// 登录信息表
#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct UserAuth {
    pub id: i32,
    pub uid: i32,
    pub identity_type: i32,
    pub identifier: String,
    pub certificate: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, Insertable)]
#[table_name = "user_auth"]
pub struct UserAuthInputDto {
    pub uid: i32,
    pub identity_type: i32,
    pub identifier: String,
    pub certificate: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

/// 返回的[user_auth]实体,排除密码
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct AuthResponse {
    pub id: i32,
    pub uid: i32,
    pub identity_type: i32,
    pub identifier: String,
}

impl From<UserAuth> for AuthResponse {
    fn from(user: UserAuth) -> Self {
        AuthResponse {
            id: user.id,
            uid: user.uid,
            identity_type: user.identity_type,
            identifier: user.identifier,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuthClaim {
    /// user id, the user-base id
    pub uid: i32,
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

pub fn find_by_3rd(
    pool: &PoolType,
    ident: &str,
    account_type: i32,
    nick:&str,
) -> Result<(AuthResponse, UserBase), ServiceError> {
    use crate::models::user::base::UserBaseInputDto;
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
                        .filter(id.eq(dto.id))
                        .first::<UserBase>(&conn)
                        .map_err(|_| ServiceError::Unauthorized("Invalid login".into()))?;
                    Ok((dto.into(), base))
                }
                Err(_) => {
                    let dto = UserBaseInputDto {
                        user_role: 2,
                        register_source: 3,
                        nick_name: nick, // diesel::insert_into(use_base)
                    };

                    let dto  = insert_into(user_base)
                        .values(&dto)
                        .get_result::<UserBase>(&conn)
                        .map_err(|err| ServiceError::PoolError(err.to_string()))?;

                    let dto = UserAuthInputDto {
                        uid: dto.id,
                        identity_type: 3,
                        identifier: ident.into(),
                        certificate: "".into(),
                        created_at: Utc::now().naive_utc(),
                        updated_at: Utc::now().naive_utc(),
                    };

                    let dto = insert_into(user_auth)
                        .values(&dto)
                        ..get_result::<UserAuth>(&conn)
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
// }

#[cfg(test)]
pub mod tests {

    use super::*;
    use crate::tests::helpers::tests::get_pool;
}
