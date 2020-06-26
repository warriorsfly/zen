use chrono::NaiveDateTime;
use uuid::Uuid;

use crate::schema::users;

#[derive(Debug, Deserialize, Queryable, Identifiable, Serialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password: String,
    pub bio: Option<String>,
    pub avatar: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    // pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password: String,
    pub bio: Option<String>,
    pub avatar: Option<String>,
}

#[derive(Debug, AsChangeset)]
#[table_name = "users"]
pub struct UserChange {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub bio: Option<String>,
    pub avatar: Option<String>,
}
