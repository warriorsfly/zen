use crate::{
    config::db::Pool,
    constants,
    error::ServiceError,
    models::user::{UserAuth, UserDTO, LoginDTO},
    models::user_token::UserToken,
    utils::token_utils,
};
use actix_web::{
    http::{
        StatusCode,
        header::HeaderValue,
    },
    web,
};

#[derive(Serialize, Deserialize)]
pub struct TokenResponse {
    pub token:String,
    pub token_type:String,
}

// pub fn signup(user)