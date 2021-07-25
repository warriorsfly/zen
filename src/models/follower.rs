use chrono::{DateTime, Utc};
use crate::schema::followers;

#[derive(Debug, Queryable, Identifiable)]
#[primary_key(user_id, follower_id)]
pub struct Follower {
    pub user_id: i32,
    pub follower_id: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Insertable)]
#[table_name = "followers"]
pub struct NewFollower {
    pub user_id: i32,
    pub follower_id: i32,
}
