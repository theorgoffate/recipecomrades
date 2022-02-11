use juniper::{graphql_object, FieldResult};

pub struct Query;

#[graphql_object()]
impl Query {
    pub fn apiVersion() -> &'static str {
        "1.0"
    }
}
