use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::User;
use crate::{config::DATE_FORMAT, schema::comments};

#[derive(Debug, Deserialize, Queryable, Identifiable)]
pub struct Comment {
    pub id: i32,
    pub article_id: i32,
    pub author_id: i32,
    pub body: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Comment {
    pub fn attach(self, author: User) -> CommentJson {
        CommentJson {
            id: self.id,
            body: self.body,
            author,
            created_at: self.created_at.format(DATE_FORMAT).to_string(),
            updated_at: self.updated_at.format(DATE_FORMAT).to_string(),
        }
    }
}

#[derive(Serialize)]
pub struct CommentJson {
    pub id: i32,
    pub body: String,
    pub author: User,
    pub created_at: String,
    pub updated_at: String,
}
