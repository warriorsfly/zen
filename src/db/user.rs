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
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{auth::hash, tests::helpers::tests::get_pool};

    #[test]
    fn create_user() {
        let pool = get_pool();
        let password = hash("love22222222");
        let walker = NewUser {
            username: "Allen",
            email: "warriorsfly@gmail.com",
            password: &password,
        };
        let res = create(&pool, &walker);

        assert!(res.is_ok());
    }
}
