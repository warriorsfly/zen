use crate::models::{NewUser, User, UserChange};

use crate::schema::users::{self, dsl::*};
use diesel::{insert_into, prelude::*, update};
use zen_database::DatabaseConnection;

/// create user
pub fn create_user<'a>(conn: &'a DatabaseConnection, item: &'a NewUser) -> QueryResult<User> {
    insert_into(users).values(item).get_result::<User>(conn)
}

// pub fn find_users(pool: &PoolType) -> Result<Vec<User>, ServiceError> {
//     // use crate::schema::users::{self, dsl::*};
//     let conn = pool.get()?;
//     users.
//         .get_result::<User>(conn)
//         .map_err(|err| ServiceError::DataBaseError(err.to_string()))
// }

pub fn find_user_by_id<'a>(conn: &'a DatabaseConnection, uid: &'a i32) -> QueryResult<User> {
    users.find(uid).get_result::<User>(conn)
}

pub fn find_by_email<'a>(
    conn: &'a DatabaseConnection,
    em: &'a str,
    pa: &'a str,
) -> QueryResult<User> {
    users
        .filter(email.eq(em))
        .filter(password.eq(pa))
        .limit(1)
        .get_result::<User>(conn)
}

pub fn update_user<'a>(
    conn: &'a DatabaseConnection,
    uid: &'a i32,
    item: &'a UserChange,
) -> QueryResult<User> {
    let _user = users.find(uid).get_result::<User>(conn)?;
    update(users::table).set(item).get_result::<User>(conn)
}
