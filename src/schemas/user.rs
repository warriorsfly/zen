use super::DataSource;
use crate::database::User;
use juniper::{graphql_object, GraphQLInputObject};
use validator::Validate;

#[graphql_object(Context = DataSource)]
impl User {
    pub fn id(&self) -> &i32 {
        &self.id
    }
    #[graphql(description = "user's name")]
    pub fn username(&self) -> &str {
        &self.username
    }

    #[graphql(description = "user's email")]
    pub fn email(&self) -> &str {
        &self.email
    }
    #[graphql(description = "user's bio")]
    pub fn bio(&self) -> &str {
        &self.bio
    }
    #[graphql(description = "user's avatar")]
    pub fn avatar(&self) -> &str {
        &self.avatar
    }
}

#[derive(Debug, GraphQLInputObject, Validate)]
pub struct NewUserInput {
    #[graphql(description = "user's name")]
    pub username: String,
    #[graphql(description = "user's email")]
    #[validate(email(message = "email must be a valid email"))]
    pub email: String,
    #[graphql(description = "user's password,at least 6 chapters")]
    pub password: String,
    pub bio: Option<String>,
    pub avatar: Option<String>,
}

#[derive(Debug, GraphQLInputObject)]
pub struct UpdateUserInput {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub bio: Option<String>,
    pub avatar: Option<String>,
}

#[derive(Debug, juniper::GraphQLInputObject, Validate)]
pub struct UserLoginInput {
    #[graphql(description = "user's email")]
    #[validate(email(message = "email must be a valid email"))]
    pub email: String,
    #[graphql(description = "user's password,at least 6 chapters")]
    pub password: String,
}
