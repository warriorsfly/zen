use actix_web::{Error, HttpRequest, HttpResponse, Responder};
use chrono;
use diesel::prelude::*;
use futures::future::{ready, Ready};
use serde::Serialize;
use uuid::Uuid;

// use crate::{
//     config::db::Connection,
//     constants,
//     entity::user::token::UserToken,
//     schema::user_auth::{self, dsl::*},
// };

//表实体
#[derive(Debug, Serialize, Deserialize, Queryable)]
// #[table_name = "user_auth"]
pub struct UserAuth {
    pub id: i32,
    pub uid: i32,
    pub identity_type: i32,
    pub identifier: String,
    pub certificate: String,
    pub login_session: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuthAccount {
    pub id: String,
    pub phone: String,
}

impl Responder for UserAuth {
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

impl UserAuth {}
