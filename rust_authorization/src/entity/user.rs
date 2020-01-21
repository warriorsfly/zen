use actix_web::{Error, HttpRequest, HttpResponse, Responder};

use futures::future::{ready, Ready};
use serde::Serialize;
use super::schema::*;
use uuid::Uuid;

#[derive(Serialize,Deserialize)]
pub struct LoginDTO {
    pub identity_type:i32,
    pub identifier:String,
    pub certificate:String,
}

#[derive(Insertable)]
#[table_name = "user_auth"]
pub struct LoginInfoDTO {
    pub identity_type: identity_type,
    pub identifier: String,
    pub login_session: String,
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
#[table_name = "user_auth"]
pub struct UserAuth {
    pub id: i32,
    pub uid: uuid::Uuid,
    pub identity_type: i32,
    pub identifier: String,
    pub certificate: String,
    pub login_session: String,
    pub created_at: i64,
    pub updated_at: i64,
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
