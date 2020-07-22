use crate::{database::DatabaseConnectionPool, db, errors::ServiceError, helpers::respond_json};
use actix_web::web::{block, Data, Json};

pub async fn get_tags(
    pool: Data<DatabaseConnectionPool>,
) -> Result<Json<Vec<String>>, ServiceError> {
    let tags = block(move || {
        let conn = &pool.get()?;
        Ok(db::get_tags(conn))
    })
    .await?;

    respond_json(tags)
}
