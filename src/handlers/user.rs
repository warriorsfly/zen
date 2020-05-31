use crate::{
    database::PoolType,
    db,
    errors::ServiceError,
    helpers::respond_json,
    models::user::{NewUser, User},
};
use actix_web::web::{block, Data, Json, Path};

pub async fn create_user(
    pool: Data<PoolType>,
    dto: Json<NewUser>,
) -> Result<Json<User>, ServiceError> {
    let user = block(move || db::user::create(&pool, &dto)).await?;
    respond_json(user)
}

pub async fn get_user(
    pool: Data<PoolType>,
    user_id: Path<i32>,
) -> Result<Json<User>, ServiceError> {
    let user = block(move || db::user::get_user(&pool, *user_id)).await?;
    respond_json(user)
}
