use crate::{
    database::ConnectionPool,
    errors::ServError,
    models::{NewUser, User, UserChange},
};

use crate::schema::users::{self, dsl::*};
use diesel::{insert_into, prelude::*, update};
use uuid::Uuid;

/// create user
pub fn create_user(pool: &ConnectionPool, item: &NewUser) -> Result<User, ServError> {
    // use crate::schema::users::{self, dsl::*};
    let conn = pool.get()?;
    insert_into(users)
        .values(item)
        .get_result::<User>(&conn)
        .map_err(|err| ServError::DataBaseError(err.to_string()))
}

// pub fn find_users(pool: &PoolType) -> Result<Vec<User>, ServiceError> {
//     // use crate::schema::users::{self, dsl::*};
//     let conn = pool.get()?;
//     users.
//         .get_result::<User>(&conn)
//         .map_err(|err| ServiceError::DataBaseError(err.to_string()))
// }

pub fn find_user_by_id(pool: &ConnectionPool, uid: &Uuid) -> Result<User, ServError> {
    // use crate::schema::users::{self, dsl::*};
    let conn = pool.get()?;
    users
        .find(uid)
        .get_result::<User>(&conn)
        .map_err(|err| ServError::DataBaseError(err.to_string()))
}

pub fn find_by_email(pool: &ConnectionPool, em: &str, pa: &str) -> Result<User, ServError> {
    // use crate::schema::users::{self, dsl::*};
    let conn = pool.get()?;

    users
        .filter(email.eq(em))
        .filter(password.eq(pa))
        .limit(1)
        .get_result::<User>(&conn)
        .map_err(|err| ServError::DataBaseError(err.to_string()))
}

pub fn update_user(pool: &ConnectionPool, uid: Uuid, item: &UserChange) -> Result<User, ServError> {
    let conn = pool.get()?;
    let _user = users
        .find(uid)
        .get_result::<User>(&conn)
        .map_err(|err| ServError::DataBaseError(err.to_string()))?;
    update(users::table)
        .set(item)
        .get_result::<User>(&conn)
        .map_err(|err| ServError::DataBaseError(err.to_string()))
}
