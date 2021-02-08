use super::{
    user::{NewUserInput, UpdateUserInput},
    DataSource,
};
use crate::{
    auth::hash,
    database::{register_by_email, NewUser, User},
    schema::users::{self, dsl::*},
};
use diesel::prelude::*;
use juniper::{graphql_object, EmptySubscription, FieldError, FieldResult, RootNode};
use validator::Validate;
pub struct Query;

#[graphql_object(Context = DataSource)]
impl Query {
    #[graphql(description = "List of all users")]
    fn users(ctx: &DataSource) -> FieldResult<Vec<User>> {
        let conn = &ctx.database.get()?;
        let urs = users::table.load::<User>(conn)?;
        Ok(urs)
    }

    #[graphql(arguments(uid(description = "id of the user")))]
    fn user(ctx: &DataSource, uid: i32) -> FieldResult<User> {
        let conn = &ctx.database.get()?;
        let ur = users::table.find(uid).get_result::<User>(conn)?;
        Ok(ur)
    }
}

pub struct Mutation;

#[graphql_object(Context = DataSource)]
impl Mutation {
    #[graphql(description = "sign up a new user by email")]
    fn register_email(ctx: &DataSource, entity: NewUserInput) -> FieldResult<User> {
        entity.validate()?;
        let conn = &ctx.database.get()?;
        let psw = hash(&entity.password);
        let ur = NewUser {
            username: &entity.username,
            email: &entity.email,
            password: &psw,
            bio: &entity.bio.unwrap_or("".into()),
            avatar: &entity.avatar.unwrap_or("".into()),
        };
        register_by_email(conn, ur).map_err(|err| FieldError::from(err.to_string()))
    }

    // #[graphql(description = "login into system")]
    // fn login(ctx: &DataSource, entity: NewUser) -> FieldResult<User> {
    //     let conn = &ctx.database.get()?;
    //     let ur = diesel::insert_into(users)
    //         .values(entity)
    //         .get_result::<User>(conn)
    //         .expect("insert user error");
    //     Ok(ur)
    // }
}

pub type Schema = RootNode<'static, Query, Mutation, EmptySubscription<DataSource>>;

pub(crate) fn init_schema() -> Schema {
    Schema::new(Query, Mutation, EmptySubscription::<DataSource>::new())
}
