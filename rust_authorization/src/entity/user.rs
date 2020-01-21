use crate::{
    config::db::Connection,
    constants,
    entity::{
        login_history::LoginHistory,
        user_token::UserToken,
    },
    schema::user_auth::{self, dsl::*},
};
use bcrypt::{hash, verify, DEFAULT_COST};
use diesel::prelude::*;
use uuid::Uuid;

//登陆
#[derive(Serialize,Deserialize)]
pub struct LoginDTO {
    pub identity_type:i32,
    pub identifier:String,
    pub certificate:String,
}
//session登陆
#[derive(Insertable)]
#[table_name = "user_auth"]
pub struct LoginInfoDTO {
    pub identity_type: i32,
    pub identifier: String,
    pub login_session: String,
}
//表实体
#[derive(Debug, Serialize, Deserialize, Queryable)]
#[table_name = "user_auth"]
pub struct UserAuth {
    pub id: i32,
    pub uid: String,
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

impl UserAuth {
    pub fn find_user_by_identifier(ty: i32, id:&str, conn:&Connection)->QueryResult<UserAuth>{
       user_auth.filter(identity_type.eq(ty) && identifier.eq(id))

    }
}
