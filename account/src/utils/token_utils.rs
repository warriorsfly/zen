use crate::config::db::Pool;

use jsonwebtoken::{errors, decode, Validation, DecodingKey,TokenData};
use crate::{
    entity::{
        user::auth::UserAuth,
        user::token::{KEY, UserToken},
    },
};

use actix_web::web;

pub fn decode_token(token:String)-> errors::Result<TokenData<UserToken>>{
    decode::<UserToken>(&token,&DecodingKey::from_secret(&KEY),&Validation::default())
}

pub fn verify_token(token:&TokenData<UserToken>,pool:&web::Data<Pool>)->Result<String,String>{
    if UserAuth::is_valid_login_session(&token.claims, &pool.get().unwrap()) {
        Ok(token.claims.identifier.to_string())
    }else{
        Err("Invalid token".to_string())
    }
}
