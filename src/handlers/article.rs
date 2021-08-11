use crate::{
    database::{self, DatabaseConnectionPool},
    errors::ZnError,
    helpers::{respond_json, respond_ok},
    models::ArticleJson,
    models::CommentJson,
    validate::validate,
};
use actix_web::{
    body::Body,
    web::{block, Data, Form, HttpResponse, Json, Path},
};
use database::{ArticleFindData, FeedArticleData, UpdateArticleData};
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
    params: Json<NewArticle>,
) -> Result<Json<ArticleJson>, ZnError> {
    validate(&params)?;
    let new_article = block(move || {
        database::create_article(
            &pool,
            &1,
            // &1,
            &params.title,
            &params.description,
            &params.body,
            &params.tags,
        )
    })
    .await??;
    respond_json(new_article)
}

pub async fn search_articles(
    pool: Data<DatabaseConnectionPool>,
    params: Form<ArticleFindData>,
) -> Result<Json<(Vec<ArticleJson>, i64)>, ZnError> {
    // validate(&params)?;
    let articles =
        block(move || database::search_articles(&pool.into_inner(), Some(1), &params)).await??;
    respond_json(articles)
}

pub async fn get_one_article(
    pool: Data<DatabaseConnectionPool>,
    slug: Path<String>,
) -> Result<Json<ArticleJson>, ZnError> {
    let article = block(move || database::find_one_article(&pool, &slug, &1)).await??;

    respond_json(article)
}

// pub async fn favorite_article(
//     pool: Data<DatabaseConnectionPool>,
//     slug: Path<String>,
// ) -> Result<Json<ArticleJson>, ServError> {
//     let article = block(move || database::favorite_article(&pool, &slug, &1)).await??;

//     respond_json(article)
// }

// pub async fn unfavorite_article(
//     pool: Data<DatabaseConnectionPool>,
//     slug: Path<String>,
// ) -> Result<Json<ArticleJson>, ServError> {
//     let article = block(move || database::unfavorite_article(&pool, &slug, &1)).await??;

//     respond_json(article)
// }

pub async fn feed_articles(
    pool: Data<DatabaseConnectionPool>,
    // redis: Cache,
    // claim: Claims,
    slug: Form<FeedArticleData>,
) -> Result<Json<Vec<ArticleJson>>, ZnError> {
    let articles = block(move || database::feed_article(&pool, slug.into_inner(), &1)).await??;

    respond_json(articles)
}

pub async fn update_article(
    pool: Data<DatabaseConnectionPool>,
    // redis: Cache,
    // claim: Claims,
    slug: Path<String>,
    params: Json<UpdateArticleData>,
) -> Result<Json<ArticleJson>, ZnError> {
    // validate(&params)?;
    let article =
        block(move || database::update_article(&pool, slug.as_ref(), &1, params.into_inner()))
            .await??;

    respond_json(article)
}

pub async fn delete_article(
    pool: Data<DatabaseConnectionPool>,
    // redis: Cache,
    // claim: Claims,
    slug: Path<String>,
) -> Result<HttpResponse<Body>, ZnError> {
    // validate(&params)?;
    block(move || database::delete_article(&pool, slug.as_ref(), &1)).await??;
    respond_ok()
}

pub async fn create_comment(
    pool: Data<DatabaseConnectionPool>,
    // claim: Claims,
    slug: String,
    body: String,
) -> Result<Json<CommentJson>, ZnError> {
    let comment =
        block(move || database::create_comment(&pool, &1, slug.as_ref(), body.as_ref())).await??;
    respond_json(comment)
}

pub async fn find_comments_by_slug(
    pool: Data<DatabaseConnectionPool>,
    // claim: Claims,
    slug: String,
) -> Result<Json<Vec<CommentJson>>, ZnError> {
    let comments = block(move || database::find_comments_by_slug(&pool, slug.as_ref())).await??;
    respond_json(comments)
}

pub async fn delete_comment(
    pool: Data<DatabaseConnectionPool>,
    // claim: Claims,
    slug: String,
    comment_id: String,
) -> Result<String, ZnError> {
    block(move || {
        database::delete_comment(
            &pool,
            &1,
            slug.as_ref(),
            &comment_id.parse::<i32>().unwrap(),
        )
    })
    .await??;
    Ok("success".to_string())
}
