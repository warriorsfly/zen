
use crate::schema::user_base::{self, dsl::*};

use actix_web::{Error, HttpRequest, HttpResponse, Responder};
use chrono;
use diesel::prelude::*;

use futures::future::{ready, Ready};
use serde::Serialize;

// use crate::{
//     config::db::Connection,
//     constants,
//     schema::user_base::{self, dsl::*},
// };

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[table_name = "user_base"]
pub struct UserBaseDto<'a> {
    pub id: &'a str,
    pub user_role: i32,
    pub register_source: i32,
    pub nick_name: &'a str,
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct UserBase {
    pub id: String,
    pub user_role: i32,
    pub register_source: i32,
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

impl UserBase {}
