use crate::{
    auth::PrivateClaim, cache::Cache, database::DatabaseConnectionPool, db, errors::ServiceError,
    helpers::respond_json, models::ArticleJson, validate::validate,
};
use actix_web::web::{block, Data, Json};
use db::ArticleFindData;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct NewArticle {
    #[validate(length(min = 1))]
    title: String,
    #[validate(length(min = 1))]
    description: String,
    #[validate(length(min = 1))]
    body: String,
    tags: Vec<String>,
}
pub struct ArticleRequest {}
pub async fn post_article(
    pool: Data<DatabaseConnectionPool>,
    redis: Cache,
    claim: PrivateClaim,
    params: Json<NewArticle>,
) -> Result<Json<ArticleJson>, ServiceError> {
    validate(&params)?;
    let new_article = block(move || {
        db::create_article(
            &pool,
            &claim.id,
            &params.title,
            &params.description,
            &params.body,
            &params.tags,
        )
    })
    .await?;
    respond_json(new_article)
}

pub async fn search_article(
    pool: Data<DatabaseConnectionPool>,
    redis: Cache,
    claim: PrivateClaim,
    params: Json<ArticleFindData>,
) -> Result<Json<(Vec<ArticleJson>, i64)>, ServiceError> {
    // validate(&params)?;
    let articles = block(move || db::search(&pool, Some(claim.id), &params)).await?;
    respond_json(articles)
}
