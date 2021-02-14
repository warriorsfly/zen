use super::DataSource;
use crate::database::{find_user_by_id, Article, User};
use chrono::{DateTime, Utc};
use juniper::{graphql_object, FieldResult, GraphQLInputObject};
use serde::Deserialize;
use validator::Validate;

// pub struct Article {
//     pub id: i32,
//     pub author_id: i32,
//     pub slug: String,
//     pub title: String,
//     pub description: String,
//     pub body: String,
//     pub tag_list: Vec<String>,
//     pub favorites_count: i32,
//     pub created_at: DateTime<Utc>,
//     pub updated_at: DateTime<Utc>,
// }

#[graphql_object(Context = DataSource)]
impl Article {
    pub fn id(&self) -> &i32 {
        &self.id
    }

    pub fn author(&self, ctx: &DataSource) -> FieldResult<User> {
        let conn = &ctx.database.get()?;
        let user = find_user_by_id(conn, &self.id)?;
        Ok(user)
    }

    pub fn slug(&self) -> &str {
        &self.slug
    }
    pub fn title(&self) -> &str {
        &self.title
    }
    pub fn description(&self) -> &str {
        &self.description
    }
    pub fn body(&self) -> &str {
        &self.body
    }
    pub fn tag_list(&self) -> &Vec<String> {
        &self.tag_list
    }

    pub fn favorites_count(&self) -> &i32 {
        &self.favorites_count
    }

    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }

    pub fn updated_at(&self) -> &DateTime<Utc> {
        &self.updated_at
    }
}

#[derive(Deserialize, GraphQLInputObject, Validate)]
pub struct NewArticleInput {
    #[validate(length(min = 1))]
    #[graphql(description = "the title of the article")]
    title: String,
    #[validate(length(min = 1))]
    description: String,
    #[validate(length(min = 1))]
    body: String,
    tags: Vec<String>,
}
