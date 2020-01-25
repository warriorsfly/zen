use bcrypt::{hash, verify, DEFAULT_COST};
use diesel::prelude::*;
use actix_web::{Error, HttpRequest, HttpResponse, Responder};
use futures::future::{ready, Ready};
use serde::Serialize;
use chrono;
use uuid::Uuid;

use crate::{
    config::db::Connection,
    constants,
    entity::{
        // login_history::LoginHistory,
        user_token::UserToken,
    },
    schema::{
        user_auth::{self, dsl::*},
        user_base::{self, dsl::*},
        user_extra::{self, dsl::*},
        user_location::{self, dsl::*},
    }
};

//注册
#[derive(Serialize,Deserialize)]
pub struct RegDTO {
    pub identity_type:i32,
    pub identifier:String,
    pub certificate:String,
}
//注册
#[derive(Insertable,Serialize,Deserialize)]
#[table_name = "user_auth"]
pub struct UserDTO<'a> {
    pub uid: Uuid,
    pub identity_type:i32,
    pub identifier:&'a str,
    pub certificate:&'a str,
}
//登陆
#[derive(Serialize,Deserialize)]
pub struct LoginDTO {
    pub identifier:String,
    pub certificate:String,
}
//session登陆
#[derive(Insertable)]
#[table_name = "user_auth"]
pub struct LoginResultDTO {
    pub identity_type: i32,
    pub identifier: String,
    pub login_session: String,
}
//表实体
#[derive(Debug, Insertable,Serialize, Deserialize, Queryable)]
#[table_name = "user_auth"]
pub struct UserAuth {
    pub id: i32,
    pub uid: Uuid,
    pub identity_type: i32,
    pub identifier: String,
    pub certificate: String,
    pub login_session: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
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
    // 注册
    pub fn signup(dto: RegDTO, conn:&Connection)->Result<String,String>{
        if Self::find_user_by_identifier(&dto.identifier, conn).is_err() {
            let hashed_pwd = hash(&dto.certificate, DEFAULT_COST).unwrap();
            let dto = UserDTO{
                uid:Uuid::new_v4(),
                identity_type:dto.identity_type,
                identifier:&dto.identifier,
                certificate:&hashed_pwd,
            };
            diesel::insert_into(user_auth).values(&dto).execute(conn).unwrap();
            Ok(constants::MESSAGE_SIGNUP_SUCCESS.to_string())
        }else{
            Err(format!("User '{}' is already registered", &dto.identifier))
        }
    }

    pub fn login(login:LoginDTO,conn:&Connection)->Option<LoginResultDTO> {
        let user_to_verify = user_auth
        .filter(identifier.eq(&login.identifier))
        .get_result::<UserAuth>(conn)
        .unwrap();

        if !user_to_verify.certificate.is_empty() && verify(&login.certificate, &user_to_verify.certificate).unwrap() {
            let login_session_str = UserAuth::generate_login_session();
            if UserAuth::update_login_session_to_db(&user_to_verify.identifier, &login_session_str, conn) {
                return Some(LoginResultDTO {
                    identity_type:user_to_verify.identity_type,
                    identifier: user_to_verify.identifier,
                    login_session: login_session_str,
                });
            }        
        }    
        None
    }

    // 退出
    pub fn logout(user_id: i32, conn: &Connection) {
        if let Ok(user) = user_auth.find(user_id).get_result::<UserAuth>(conn) {
            Self::update_login_session_to_db(&user.identifier, "", conn);
        }
    }

    // 根据id获取用户信息
    pub fn find_user_by_identifier(iden:&str, conn:&Connection)->QueryResult<UserAuth>{
       user_auth.filter(identifier.eq(iden)).get_result::<UserAuth>(conn)
    }

    // 判断登陆状态
    pub fn is_valid_login_session(token:&UserToken,conn:&Connection)->bool{
        user_auth.filter(identity_type.eq(&token.identity_type)).filter(identifier.eq(&token.identifier))
        .get_result::<UserAuth>(conn)
        .is_ok()
    }

    // 更新登陆session
    pub fn update_login_session_to_db(iden:&str, login_session_str: &str,  conn:&Connection)->bool{
        if let Ok(user) = UserAuth::find_user_by_identifier(iden, conn){
            diesel::update(user_auth.find(user.id))
            .set(login_session.eq(login_session_str.to_string()))
            .execute(conn)
            .is_ok()
        }else{
            false
        }
    }

    // 创建session
    pub fn generate_login_session() -> String {
        Uuid::new_v4().to_string()
    }
}


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

#[derive(Debug,Insertable, Serialize, Deserialize, Queryable)]
#[table_name = "user_extra"]
pub struct UserExtra {
    pub uid: uuid::Uuid,
    pub vendor: String,
    pub client_name: String,
    pub client_version: String,
    pub os_name: String,
    pub os_version: String,
    pub device_id: String,
    pub device_name: String,
    pub idfa: String,
    pub idfv: String,
    pub market: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub extend1: String,
    pub extend2: String,
    pub extend3: String
}

#[derive(Debug, Insertable, Serialize, Deserialize, Queryable)]
#[table_name = "user_location"]
pub struct UserLocation {
    pub uid: uuid::Uuid,
    pub curr_nation: String,
    pub curr_province: String,
    pub curr_city: String,
    pub curr_district: String,
    pub location: String,
    pub longitude: f64,
    pub latitude: f64,
    pub updated_at: chrono::NaiveDateTime
}