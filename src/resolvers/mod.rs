use juniper::{EmptyMutation, EmptySubscription, RootNode};
use rocket::State;

pub mod query;

pub type Schema = RootNode<'static, query::Query, EmptyMutation, EmptySubscription>;

#[post("/graphql", data = "<request>")]
pub fn graphql_post(
    request: juniper_rocket::GraphQLRequest,
    schema: &State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute_sync(schema, &())
}
