use crate::schema::users;
use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Deserialize, Queryable, Identifiable, Serialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub bio: Option<String>,
    pub avatar: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Insertable)]
#[table_name = "users"]
pub struct NewUser {
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
