use crate::schema::users::{self, dsl::*};
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use ins_database::ConnectionPool;
use serde::{Deserialize, Serialize};
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

#[derive(Debug, AsChangeset)]
#[table_name = "users"]
pub struct UpdateUser<'a> {
    pub username: &'a str,
    pub email: &'a str,
    pub password: &'a str,
    pub bio: &'a str,
    pub avatar: &'a str,
}

pub fn register_by_email<'a>(conn: &'a ConnectionPool, entity: NewUser) -> QueryResult<User> {
    diesel::insert_into(users)
        .values(entity)
        .get_result::<User>(conn)
}

pub fn find_by_email<'a>(conn: &'a ConnectionPool, em: &'a str, pa: &'a str) -> QueryResult<User> {
    // use crate::schema::users::{self, dsl::*};
    users
        .filter(email.eq(em))
        .filter(password.eq(pa))
        .limit(1)
        .get_result::<User>(conn)
}
