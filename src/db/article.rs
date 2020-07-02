use crate::{
    database::PoolType,
    errors::ServiceError,
    models::{Article, NewArticle},
    schema::{
        articles::{self, dsl::*},
        users::{self, dsl::*},
    },
};
use diesel::{delete, insert_into, prelude::*, update};
use uuid::Uuid;

/// 创建文章
pub fn create_article(
    pool: &PoolType,
    uid: Uuid,
    article: &NewArticle,
) -> Result<usize, ServiceError> {
    let conn = pool.get()?;

    insert_into(articles)
        .values(article)
        .execute(&conn)
        .map_err(|err| ServiceError::DataBaseError(err.to_string()))
}
