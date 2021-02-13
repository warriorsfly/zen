use serde::Deserialize;
use validator::Validate;
#[derive(Deserialize, Validate)]
pub struct NewArticleInput {
    #[validate(length(min = 1))]
    title: String,
    #[validate(length(min = 1))]
    description: String,
    #[validate(length(min = 1))]
    body: String,
    tags: Vec<String>,
}
