use actix_web::{Error, HttpRequest, HttpResponse, Responder};
use futures::future::{ready, Ready};
use serde::Serialize;
use chrono;
use uuid::Uuid;

use crate::{
    config::db::Connection,
    schema::{
        user_base::{self, dsl::*},
    }
};

#[derive(Debug, Insertable, Serialize, Deserialize, Queryable)]
#[table_name = "user_base"]
pub struct UserBase {
    pub uid: uuid::Uuid,
    pub user_role: i32,
    pub register_source: i32,
    pub user_name: String,
    pub nick_name: String,
    pub gender: i32,
    pub birthday: chrono::NaiveDateTime,
    pub signature: String,
    pub mobile: String,
    pub mobile_bind_time: chrono::NaiveDateTime,
    pub email: String,
    pub email_bind_time: chrono::NaiveDateTime,
    pub avatar: String,
    pub avatar200: String,
    pub avatar_source: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
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
