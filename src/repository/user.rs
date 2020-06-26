use crate::{
    database::PoolType,
    errors::ServiceError,
    models::{NewUser, User, UserChange},
};

use crate::schema::users::{self, dsl::*};
use diesel::{insert_into, prelude::*, update};

/// create user
pub fn create_user(pool: &PoolType, item: &NewUser) -> Result<User, ServiceError> {
    // use crate::schema::users::{self, dsl::*};
    let conn = pool.get()?;
    insert_into(users)
        .values(item)
        .get_result::<User>(&conn)
        .map_err(|err| ServiceError::PoolError(err.to_string()))
}

pub fn update_user(pool: &PoolType, uid: &str, item: &UserChange) -> Result<User, ServiceError> {
    let conn = pool.get()?;

    update(users::table)
        .set(item)
        .get_result::<User>(&conn)
        .map_err(|err| ServiceError::PoolError(err.to_string()))
}
