use serde::Serialize;
use validator::Validate;

use crate::schema::cms_article::{self, dsl::*};

#[derive(Clone, Debug, Deserialize, Serialize, Validate, Queryable, Identifiable, Insertable)]
#[table_name = "cms_article"]
pub struct Article {
    pub id: String,
    pub channel_id: String,
    pub author: String,
    pub title: String,
    pub content: String,
    pub image: Option<String>,
}
