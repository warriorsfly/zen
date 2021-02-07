use super::DataSource;
use crate::{
    database::user::User,
    schema::users::{self, dsl::*},
};

use juniper::graphql_object;

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

    pub fn bio(&self) -> &str {
        &self.bio
    }

    pub fn avatar(&self) -> &str {
        &self.avatar
    }
}

#[derive(Debug, juniper::GraphQLInputObject)]
pub struct NewUserInput {
    pub username: String,
    pub email: String,
    pub password: String,
    pub bio: Option<String>,
    pub avatar: Option<String>,
}

#[derive(Debug, juniper::GraphQLInputObject)]
pub struct UpdateUserInput {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub bio: Option<String>,
    pub avatar: Option<String>,
}
