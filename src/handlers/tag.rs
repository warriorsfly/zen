use crate::{database, errors::ZenError, helpers::respond_json};
use actix_web::web::{block, Data, Json};
// use zen_database::DatabaseConnectionPool;

// pub async fn get_tags(pool: Data<DatabaseConnectionPool>) -> Result<Json<Vec<String>>, ServError> {
//     let tags = block(move || database::get_tags(&pool)).await??;

//     respond_json(tags)
// }
