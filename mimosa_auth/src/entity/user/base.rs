use actix_web::{Error, HttpRequest, HttpResponse, Responder};
use chrono;
use diesel::prelude::*;
use futures::future::{ready, Ready};
use serde::Serialize;

use crate::{
    config::db::Connection,
    constants,
    schema::user_base::{self, dsl::*},
};

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[table_name = "user_base"]
pub struct UserBaseDto<'a> {
    pub user_role: i32,
    pub register_source: i32,
    pub user_name: &'a str,
    pub nick_name: &'a str,
    pub gender: i32,
    pub birthday: chrono::NaiveDateTime,
    pub signature: &'a str,
    pub mobile: &'a str,
    pub mobile_bind_time:Option<chrono::NaiveDateTime>,
    pub email: &'a str,
    pub email_bind_time:Option<chrono::NaiveDateTime>,
    pub avatar: &'a str,
    pub avatar200: &'a str,
    pub avatar_source: &'a str,
    pub created_at:chrono::NaiveDateTime,
    pub updated_at:Option<chrono::NaiveDateTime>,
    pub push_token: &'a str,
}

#[derive(Debug, Insertable,Serialize, Deserialize, Queryable)]
#[table_name = "user_base"]
pub struct UserBase {
    pub id: i32,
    pub user_role: i32,
    pub register_source: i32,
    pub user_name: String,
    pub nick_name: String,
    pub gender: i32,
    pub birthday: chrono::NaiveDateTime,
    pub signature: String,
    pub mobile: String,
    pub mobile_bind_time: Option<chrono::NaiveDateTime>,
    pub email: String,
    pub email_bind_time: Option<chrono::NaiveDateTime>,
    pub avatar: String,
    pub avatar200: String,
    pub avatar_source: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub push_token: String,
}

impl Responder for UserBase {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();

        // Create response and set content type
        ready(Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body)))
    }
}

impl UserBase {
    pub fn insert(dto: UserBaseDto, conn:&Connection)->Result<UserBase,String>{
        match diesel::insert_into(user_base).values(&dto).get_result(conn){
            Ok(result)=>Ok(result),
            Err(_)=>Err(constants::MESSAGE_CAN_NOT_INSERT_DATA.to_string())
        }
    }
    // 根据uid获取用户信息
    pub fn find_user_by_id(userId: i32, conn:&Connection)->QueryResult<UserBase>{
       user_base.filter(id.eq(userId)).get_result::<UserBase>(conn)
    }
}
