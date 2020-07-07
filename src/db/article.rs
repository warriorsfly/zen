use crate::{
    database::PoolType,
    errors::ServiceError,
    models::Article,
    schema::{articles, users},
};
use diesel::{delete, insert_into, prelude::*, update};
use uuid::Uuid;

#[derive(Debug, Insertable)]
#[table_name = "articles"]
pub struct NewArticle<'a> {
    pub author_id: &'a Uuid,
    pub slug: &'a str,
    pub title: &'a str,
    pub description: &'a str,
    pub body: &'a str,
    pub tag_list: &'a Vec<String>,
}

#[derive(Debug, AsChangeset)]
#[table_name = "articles"]
pub struct ArticleChange {
    pub slug: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub body: Option<String>,
}

#[derive(Debug, Insertable)]
#[table_name = "favorite_articles"]
pub struct NewFavoriteArticle {
    pub user_id: Uuid,
    pub article_id: Uuid,
}

/// 创建文章
pub fn create_article(
    pool: &PoolType,
    author: &Uuid,
    slug: &str,
    title: &str,
    description: &str,
    body: &str,
    tags: &Vec<String>,
) -> Result<usize, ServiceError> {
    let conn = pool.get()?;
    let new_article = NewArticle {
        author_id: author,
        slug,
        title,
        description,
        body,
        tag_list: tags,
    };
    insert_into(articles)
        .values(new_article)
        .execute(&conn)
        .map_err(|err| ServiceError::DataBaseError(err.to_string()))
}
