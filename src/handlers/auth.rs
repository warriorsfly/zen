use crate::auth::{create_jwt, hash, PrivateClaim};
use crate::database::PoolType;
use crate::errors::ServiceError;
use crate::handlers::user::UserBaseResponse;
use crate::validate::validate;

use actix_identity::Identity;
use actix_web::web::{block, Data, HttpResponse, Json};
use serde::Serialize;
use validator::Validate;

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct LoginRequest {
    pub username: String,

    #[validate(length(
        min = 6,
        max = 16,
        message = "certificate is required and must be at least 6 characters,at most 16 characters"
    ))]
    pub password: String,

    pub identity_type: i32,
}

// pub async fn login(
//     id: Identity,
//     pool: Data<PoolType>,
//     params: Json<LoginRequest>,
// ) -> Result<Json<UserBaseResponse>, ServiceError> {
//     validate(&params)?;
//     let hashed = hash(&params.password);
//     let auth = block(move || )
// }
