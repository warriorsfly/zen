use crate::{
    config::db::Pool,
    constants,
    error::ServiceError,
    entity::user::{UserAuth, LoginDTO,RegDTO},
    entity::user_token::UserToken,
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


pub fn signup(user: RegDTO, pool: &web::Data<Pool>) -> Result<String, ServiceError>{
    match UserAuth::signup(user, &pool.get().unwrap()) {
        Ok(message) => Ok(message),
        Err(message) => Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, message))
    }
}

pub fn login(login: LoginDTO, pool: &web::Data<Pool>) -> Result<TokenResponse, ServiceError>{
    match UserAuth::login(login, &pool.get().unwrap()) {
        Some(logged_user) => {
            match serde_json::from_value(json!({ "token": UserToken::generate_token(logged_user), "token_type": "bearer" })) {
                Ok(token_res) => Ok(token_res),
                Err(_) => Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, constants::MESSAGE_INTERNAL_SERVER_ERROR.to_string()))
            }
        },
        None => Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, constants::MESSAGE_LOGIN_FAILED.to_string()))
    }
}

pub fn logout(authen_header: &HeaderValue, pool: &web::Data<Pool>) -> Result<(), ServiceError>{
    if let Ok(authen_str) = authen_header.to_str() {
        if authen_str.starts_with("bearer") {
            let token = authen_str[6..authen_str.len()].trim();
            if let Ok(token_data) = token_utils::decode_token(token.to_string()) {
                if let Ok(iden) = token_utils::verify_token(&token_data, pool) {
                    if let Ok(user) = UserAuth::find_user_by_identifier(&iden, &pool.get().unwrap()) {
                        UserAuth::logout(user.id, &pool.get().unwrap());
                        return Ok(());
                    }
                }
            }
        }
    }

    Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, constants::MESSAGE_PROCESS_TOKEN_ERROR.to_string()))
}