use crate::{
    auth::PrivateClaim,
    // cache::Cache,
    database::DatabaseConnectionPool,
    db,
    errors::ServiceError,
    helpers::{respond_json, respond_ok},
    models::ArticleJson,
    validate::validate,
};
use actix_web::{
    web::{block, Data, Form, Json, Path},
    HttpResponse,
};
use db::{ArticleFindData, FeedArticleData, UpdateArticleData};
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
pub async fn create_article(
    pool: Data<DatabaseConnectionPool>,
    // redis: Cache,
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

pub async fn search_articles(
    pool: Data<DatabaseConnectionPool>,
    // redis: Cache,
    claim: PrivateClaim,
    params: Form<ArticleFindData>,
) -> Result<Json<(Vec<ArticleJson>, i64)>, ServiceError> {
    // validate(&params)?;
    let articles =
        block(move || db::search_articles(&pool.into_inner(), Some(claim.id), &params)).await?;
    respond_json(articles)
}

pub async fn get_one_article(
    pool: Data<DatabaseConnectionPool>,
    // redis: Cache,
    claim: PrivateClaim,
    slug: Path<String>,
) -> Result<Json<ArticleJson>, ServiceError> {
    let article = block(move || db::find_one_article(&pool, &slug, &claim.id)).await?;

    respond_json(article)
}

pub async fn favorite_article(
    pool: Data<DatabaseConnectionPool>,
    // redis: Cache,
    claim: PrivateClaim,
    slug: Path<String>,
) -> Result<Json<ArticleJson>, ServiceError> {
    let article = block(move || Ok(db::favorite_article(&pool, &slug, &claim.id).unwrap())).await?;

    respond_json(article)
}

pub async fn unfavorite_article(
    pool: Data<DatabaseConnectionPool>,
    // redis: Cache,
    claim: PrivateClaim,
    slug: Path<String>,
) -> Result<Json<ArticleJson>, ServiceError> {
    let article =
        block(move || Ok(db::unfavorite_article(&pool, &slug, &claim.id).unwrap())).await?;

    respond_json(article)
}

pub async fn feed_articles(
    pool: Data<DatabaseConnectionPool>,
    // redis: Cache,
    claim: PrivateClaim,
    slug: Form<FeedArticleData>,
) -> Result<Json<Vec<ArticleJson>>, ServiceError> {
    let articles =
        block(move || Ok(db::feed_article(&pool, slug.into_inner(), &claim.id))).await??;

    respond_json(articles)
}

pub async fn update_article(
    pool: Data<DatabaseConnectionPool>,
    // redis: Cache,
    claim: PrivateClaim,
    slug: Path<String>,
    params: Json<UpdateArticleData>,
) -> Result<Json<ArticleJson>, ServiceError> {
    // validate(&params)?;
    let article = block(move || {
        Ok(db::update_article(&pool, slug.as_ref(), &claim.id, params.into_inner()).unwrap())
    })
    .await?;

    respond_json(article)
}

pub async fn delete_article(
    pool: Data<DatabaseConnectionPool>,
    // redis: Cache,
    claim: PrivateClaim,
    slug: Path<String>,
) -> Result<HttpResponse, ServiceError> {
    // validate(&params)?;
    block(move || db::delete_article(&pool, slug.as_ref(), &claim.id)).await?;

    respond_ok()
}

// pub struct create_comment(pool:Data<DatabaseConnectionPool>,claim:PrivateClaim,slug:&str,)->Result<Json<Comment>,ServiceError>{
//     block(move||db::create_comment(&pool, claim.id, slug, body))
// }
