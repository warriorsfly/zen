use crate::config::db::Pool;
use crate::{
    entity::{
        user::UserAuth,
        user_token::{UserToken,KEY},
    },
};

use actix_web::web;
use jsonwebtoken::{TokenData,Validation};


pub fn decode_token(token:String)->jsonwebtoken::errors::Result<TokenData<UserToken>>{
    jsonwebtoken::decode::<UserToken>(&token,&KEY,&Validation::default())
}

pub fn verify_token(token:&TokenData<UserToken>,pool:&web::Data<Pool>)->Result<String,String>{
    if UserAuth::is_valid_login_session(&token.claims, &pool.get().unwrap()) {
        Ok(token.claims.identifier.to_string())
    }else{
        Err("Invalid token".to_string())
    }
}
