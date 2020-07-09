use crate::{
    database::DatabasePoolType,
    errors::ServiceError,
    models::{Article, ArticleJson, User},
    schema::{articles, favorite_articles, users},
};
use diesel::{insert_into, prelude::*};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use slug;
use uuid::Uuid;

const SUFFIX_LEN: usize = 6;
const DEFAULT_LIMIT: usize = 20;
#[derive(Debug, Insertable)]
#[table_name = "articles"]
struct NewArticle<'a> {
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
    pool: &DatabasePoolType,
    author: &Uuid,
    title: &str,
    description: &str,
    body: &str,
    tags: &Vec<String>,
) -> Result<ArticleJson, ServiceError> {
    let new_article = NewArticle {
        author_id: author,
        slug: &slugify(title),
        title,
        description,
        body,
        tag_list: tags,
    };
    let conn = pool.get()?;
    let author = users::table
        .find(author)
        .get_result::<User>(&conn)
        .map_err(|err| ServiceError::DataBaseError(err.to_string()))?;

    let new_article = insert_into(articles::table)
        .values(new_article)
        .get_result::<Article>(&conn)
        .map_err(|err| ServiceError::DataBaseError(err.to_string()))?;

    Ok(new_article.attach(author, 0, false))
}

fn slugify(title: &str) -> String {
    if cfg!(feature = "random-suffix") {
        format!("{}-{}", slug::slugify(title), generate_suffix(SUFFIX_LEN))
    } else {
        slug::slugify(title)
    }
}

fn generate_suffix(len: usize) -> String {
    let mut rng = thread_rng();
    (0..len).map(|_| rng.sample(Alphanumeric)).collect()
}
