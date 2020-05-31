use crate::{
    database::PoolType,
    errors::ServiceError,
    models::user::{NewUser, User},
};

use diesel::prelude::*;

// create user
pub fn create(pool: &PoolType, new_user: &NewUser) -> Result<User, ServiceError> {
    use crate::schema::users::dsl::users;
    let conn = pool.get()?;

    diesel::insert_into(users)
        .values(new_user)
        .get_result::<User>(&conn)
        .map_err(|err| ServiceError::PoolError(err.to_string()))
}

pub fn get_user(pool: &PoolType, user_id: i32) -> Result<User, ServiceError> {
    use crate::schema::users::dsl::*;
    let conn = pool.get()?;
    let not_found = format!("User {} not found", user_id);
    let user = users
        .filter(id.eq(user_id))
        .first::<User>(&conn)
        .map_err(|_| ServiceError::NotFound(not_found))?;

    Ok(user)
}

pub fn update_user(pool: &PoolType, user_id: i32) -> Result<User, ServiceError> {
    use crate::schema::users::dsl::*;
    let conn = pool.get()?;
    let not_found = format!("User {} not found", user_id);
    let user = users
        .filter(id.eq(user_id))
        .first::<User>(&conn)
        .map_err(|_| ServiceError::NotFound(not_found))?;

    Ok(user)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{auth::hash, tests::helpers::tests::get_pool};

    #[test]
    fn create_or_allen_exist() {
        let pool = get_pool();

        let password = hash("love22222222").into();
        let walker = NewUser {
            username: "Allen".into(),
            email: "warriorsfly@gmail.com".into(),
            password: password,
        };

        let res = get_user(&pool, 1);
        if let Ok(us) = res {
            assert_eq!(us.username, "Allen");
        } else {
            let res = create(&pool, &walker);

            assert!(res.is_ok());
        }
    }
}
