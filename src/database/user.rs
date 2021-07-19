use crate::{
    database::DatabaseConnectionPool,
    errors::ZenError,
    models::{NewUser, User, UserChange},
};

use crate::schema::users::{self, dsl::*};
use diesel::{insert_into, prelude::*, update};

/// create user
pub fn create_user<'a>(
    pool: &'a DatabaseConnectionPool,
    item: &'a NewUser,
) -> Result<User, ZenError> {
    // use crate::schema::users::{self, dsl::*};
    let conn = &mut pool.get()?;
    insert_into(users)
        .values(item)
        .get_result::<User>(conn)
        .map_err(|err| ZenError::DataBaseError(err.to_string()))
}

// pub fn find_users(pool: &PoolType) -> Result<Vec<User>, ServiceError> {
//     // use crate::schema::users::{self, dsl::*};
//     let conn = pool.get()?;
//     users.
//         .get_result::<User>(conn)
//         .map_err(|err| ServiceError::DataBaseError(err.to_string()))
// }

pub fn find_user_by_id<'a>(
    pool: &'a DatabaseConnectionPool,
    uid: &'a i32,
) -> Result<User, ZenError> {
    // use crate::schema::users::{self, dsl::*};
    let conn = &mut pool.get()?;
    users
        .find(uid)
        .get_result::<User>(conn)
        .map_err(|err| ZenError::DataBaseError(err.to_string()))
}

pub fn find_by_email<'a>(
    pool: &'a DatabaseConnectionPool,
    em: &'a str,
    pa: &'a str,
) -> Result<User, ZenError> {
    // use crate::schema::users::{self, dsl::*};
    let conn = &mut pool.get()?;

    users
        .filter(email.eq(em))
        .filter(password.eq(pa))
        .limit(1)
        .get_result::<User>(conn)
        .map_err(|err| ZenError::DataBaseError(err.to_string()))
}

pub fn update_user<'a>(
    pool: &'a DatabaseConnectionPool,
    uid: &'a i32,
    item: &'a UserChange,
) -> Result<User, ZenError> {
    let conn = &mut pool.get()?;
    let _user = users
        .find(uid)
        .get_result::<User>(conn)
        .map_err(|err| ZenError::DataBaseError(err.to_string()))?;
    update(users::table)
        .set(item)
        .get_result::<User>(conn)
        .map_err(|err| ZenError::DataBaseError(err.to_string()))
}
