use crate::schema::users::{self, dsl::*};
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use super::ConnectionPool;
#[derive(Debug, Deserialize, Queryable, Identifiable, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub bio: String,
    pub avatar: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
#[derive(Debug, Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub email: &'a str,
    pub password: &'a str,
    pub bio: &'a str,
    pub avatar: &'a str,
}

pub fn register<'a>(conn: &'a ConnectionPool, entity: NewUser) -> User {
    diesel::insert_into(users)
        .values(entity)
        .get_result::<User>(conn)
        .expect("insert user error")
}

#[derive(Debug, AsChangeset)]
#[table_name = "users"]
pub struct UpdateUser<'a> {
    pub username: &'a str,
    pub email: &'a str,
    pub password: &'a str,
    pub bio: &'a str,
    pub avatar: &'a str,
}
