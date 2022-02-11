#[macro_use]
extern crate rocket;
#[macro_use]
extern crate juniper;

use juniper::{EmptyMutation, EmptySubscription};

mod db;
mod models;
mod resolvers;

#[launch]
fn rocket() -> rocket::Rocket<rocket::Build> {
    rocket::build()
        .manage(resolvers::Schema::new(
            resolvers::query::Query,
            EmptyMutation::new(),
            EmptySubscription::new(),
        ))
        .mount("/", routes![resolvers::graphql_post])
}
