use super::DataSource;
use crate::schema::users::{self, dsl::*};
use chrono::{DateTime, Utc};
use juniper::graphql_object;
use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Queryable, Identifiable, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub bio: Option<String>,
    pub avatar: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[graphql_object(Context = DataSource)]
impl User {
    pub fn id(&self) -> &i32 {
        &self.id
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn email(&self) -> &str {
        &self.email
    }

    pub fn bio(&self) -> &Option<String> {
        &self.bio
    }

    pub fn avatar(&self) -> &Option<String> {
        &self.avatar
    }
}
