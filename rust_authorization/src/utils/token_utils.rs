use crate::{
    entity::{
        user::UserAuth,
        user_token::{UserToken,Key},
    },
};

use actix_web::web;
use jsonwebtoken::{TokenData,Validation};


pub fn decode_token(token:String)->jsonwebtoken::errors::Result<TokenData<UserToken>>{
    jsonwebtoken::decode::<UserToken>(&token,Key,&Validation::default())
}

// pub fn verify_token(token:&TokenData<UserToken>,)