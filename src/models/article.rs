use chrono::{DateTime, Utc};;
use uuid::Uuid;
use serde::Serialize;
use crate::schema::{articles, favorite_articles};

#[derive(Queryable)]
pub struct Article {
    pub id: Uuid,
    pub author_id: Uuid,
    pub slug: String,
    pub title: String,
    pub description: String,
    pub body: String,
    pub tag_list: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Article {
    pub fn attach(self, author: User, favorited: bool) -> ArticleJson {
        ArticleJson {
            id: self.id,
            slug: self.slug,
            title: self.title,
            description: self.description,
            body: self.body,
            author,
            tags: self.tag_list,
            created_at: self.created_at.format(DATE_FORMAT).to_string(),
            updated_at: self.updated_at.format(DATE_FORMAT).to_string(),
            favorites_count: self.favorites_count,
            favorited,
        }
    }
}

#[derive(Serialize)]
pub struct ArticleJson {
    pub id: Uuid,
    pub author: User,
    pub slug: String,
    pub title: String,
    pub description: String,
    pub body: String,
    pub tags: Vec<String>,
    pub created_at: String,
    pub updated_at: String,
}
