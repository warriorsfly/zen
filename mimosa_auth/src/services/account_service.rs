use bcrypt::{hash, DEFAULT_COST};

use crate::{
    config::db::{Connection,Pool},
    constants,
    
    error::ServiceError,
    entity::user:: {
        auth::{UserAuth, LoginDTO,UserDTO},
        base::{UserBase,UserBaseDto},
        token::UserToken,
    },
    utils::token_utils,
};
use actix_web::{
    // client::Client,
    http::{
        StatusCode,
        header::HeaderValue,
    },
    web,
};

use uuid::Uuid;

//注册
#[derive(Serialize,Deserialize)]
pub struct ReqRegist {
    pub identity_type:i32,
    pub identifier:String,
    pub certificate:String,
}

#[derive(Serialize, Deserialize)]
pub struct RespToken {
    pub token:String,
    pub token_type:String,
}


// pub fn check_phone_exist(phone:String,)
pub fn signup(dto: ReqRegist, pool: &web::Data<Pool>) -> Result<String, ServiceError>{
    let conn = &pool.get().unwrap();
    if UserAuth::find_user_by_identifier(&dto.identifier,conn).is_err(){
        // 创建用户信息表
        // let baseDto = UserBaseDto{

        // };
        let hashed_pwd = hash(&dto.certificate, DEFAULT_COST).unwrap();
        let dto = UserDTO{
            uid:1,
            identity_type:dto.identity_type,
            identifier:&dto.identifier,
            certificate:&hashed_pwd,
        };
        match UserAuth::insert(dto, conn) {
                Ok(message) => {
                    Ok(message)
                },
                Err(message) => Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, message))
            }
    }else{
        Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, format!("User '{}' is already registered", &dto.identifier)))
    }
}

pub fn login(login: LoginDTO, pool: &web::Data<Pool>) -> Result<RespToken, ServiceError>{
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
