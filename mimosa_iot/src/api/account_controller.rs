use crate::{
    config::db::Pool,
    constants,
    entity::{
        user::auth::{LoginDTO},
        response::Response,
    },
    services::account_service,
};

use actix::prelude::*;
use actix_redis::{Command, RedisActor};
use actix_web::{post,web, HttpRequest, HttpResponse, Result};

// POST api/auth/signup
#[post("/signup")]
pub async fn signup(dto: web::Json<account_service::ReqRegist>, pool: web::Data<Pool>,redis: web::Data<Addr<RedisActor>>,) -> Result<HttpResponse>{
    match account_service::signup(dto.0, &pool) {
        Ok(message) => Ok(HttpResponse::Ok().json(Response::new(&message, constants::EMPTY))),
        Err(err) => Ok(err.response()),
    }
}

// POST api/auth/login
#[post("/login")]
pub async fn login(login_dto: web::Json<LoginDTO>, pool: web::Data<Pool>) -> Result<HttpResponse>{
    match account_service::login(login_dto.0, &pool) {
        Ok(token_res) => Ok(HttpResponse::Ok().json(Response::new(constants::MESSAGE_LOGIN_SUCCESS, token_res))),
        Err(err) => Ok(err.response()),
    }
}

// POST api/auth/logout
#[post("/logout")]
pub async fn logout(req: HttpRequest, pool: web::Data<Pool>) -> Result<HttpResponse>{
    if let Some(authen_header) = req.headers().get(constants::AUTHORIZATION) {
        account_service::logout(authen_header, &pool);
        Ok(HttpResponse::Ok().json(Response::new(constants::MESSAGE_LOGOUT_SUCCESS, constants::EMPTY)))
    } else {
        Ok(HttpResponse::BadRequest().json(Response::new(constants::MESSAGE_TOKEN_MISSING, constants::EMPTY)))
    }
}
