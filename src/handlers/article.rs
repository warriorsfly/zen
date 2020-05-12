use crate::errors::ServiceError;
use crate::validate::validate;
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
    title: Option<String>,
    #[validate(length(min = 1))]
    description: Option<String>,
    #[validate(length(min = 1))]
    body: Option<String>,
    #[serde(rename = "tagList")]
    tags: Vec<String>,
}

// #[post("/articles")]
// pub async fn post_articles(
//     pool: Data<PoolType>,
//     params: Json<NewArticle>,
// ) -> Result<String, ServiceError> {
//     validate(&params)?;

//     let new_article = params.into_inner().article;
// }
