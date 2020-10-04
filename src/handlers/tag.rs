use crate::{database::ConnectionPool, db, errors::ServiceError, helpers::respond_json};
use actix_web::web::{block, Data, Json};

pub async fn get_tags(pool: Data<ConnectionPool>) -> Result<Json<Vec<String>>, ServiceError> {
    let tags = block(move || db::get_tags(&pool)).await?;

    respond_json(tags)
}
