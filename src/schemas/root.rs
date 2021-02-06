use super::{user::User, DataSource};
use crate::schema::*;
use diesel::prelude::*;
use juniper::{graphql_object, EmptyMutation, EmptySubscription, FieldResult, RootNode};
pub struct Query;

#[graphql_object(Context = DataSource)]
impl Query {
    #[graphql(description = "List of all users")]
    fn users(ctx: &DataSource) -> FieldResult<Vec<User>> {
        let conn = &ctx.database.get()?;
        let users = users::table.load::<User>(conn)?;
        Ok(users)
    }

    #[graphql(arguments(id(description = "id of the provider")))]
    fn provider(ctx: &DataSource, id: i32) -> FieldResult<User> {
        let conn = &ctx.database.get()?;
        let user = users::table.find(id).get_result::<User>(conn)?;
        Ok(user)
    }
}

pub type Schema =
    RootNode<'static, Query, EmptyMutation<DataSource>, EmptySubscription<DataSource>>;

pub(crate) fn init_schema() -> Schema {
    Schema::new(
        Query,
        EmptyMutation::<DataSource>::new(),
        EmptySubscription::<DataSource>::new(),
    )
}
