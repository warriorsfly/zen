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

// pub fn verify_token(token:&TokenData<UserToken>,)