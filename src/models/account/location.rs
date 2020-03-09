use actix_web::{Error, HttpRequest, HttpResponse, Responder};
use chrono;
use diesel::prelude::*;
use futures::future::{ready, Ready};
use serde::Serialize;
use uuid::Uuid;

// use crate::{
//     config::db::Connection,
//     constants,
//     schema::user_location::{self, dsl::*},
// };
#[derive(Debug, Serialize, Deserialize, Queryable)]
// #[table_name = "user_location"]
pub struct UserLocation {
    pub uid: i32,
    pub curr_nation: String,
    pub curr_province: String,
    pub curr_city: String,
    pub curr_district: String,
    pub location: String,
    pub longitude: f64,
    pub latitude: f64,
    pub updated_at: chrono::NaiveDateTime,
}

impl Responder for UserLocation {
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
