use crate::{database::{self, DatabaseConnectionPool}, errors::ZnError, helpers::respond_json};
use actix_web::web::{block, Data, Json};

// pub async fn get_tags(pool: &DatabaseConnectionPool,) -> Result<Json<Vec<String>>, ZenError> {
//     let tags = block(move || database::get_tags(&pool)).await??;

//     respond_json(tags)
// }
