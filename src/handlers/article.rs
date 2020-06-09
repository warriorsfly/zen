use crate::db::article;
use crate::errors::ServiceError;
use crate::{
    auth::JwtAccount, database::PoolType, helpers::respond_json, models::ArticleJson,
    validate::validate,
};
use actix_web::{
    post,
    web::{Data, Json},
};
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct NewArticle {
    article: NewArticleData,
}

#[derive(Deserialize, Validate)]
pub struct NewArticleData {
    #[validate(length(min = 1))]
    title: String,
    #[validate(length(min = 1))]
    description: String,
    #[validate(length(min = 1))]
    body: String,
    #[serde(rename = "tagList")]
    tags: Vec<String>,
}

#[post("/articles")]
pub async fn post_articles(
    author: JwtAccount,
    pool: Data<PoolType>,
    params: Json<NewArticle>,
) -> Result<Json<ArticleJson>, ServiceError> {
    validate(&params)?;
    let new_article = params.into_inner().article;

    match article::create(
        pool.as_ref(),
        author.id,
        &new_article.title,
        &new_article.description,
        &new_article.body,
        &new_article.tags,
    ) {
        Ok(arti) => respond_json(arti),
        Err(e) => Err(e),
    }
}
