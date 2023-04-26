#[macro_use]
extern crate rocket;
mod api;
mod models;
mod repository;
mod routes;
use routes::*;
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/",
            routes![index_rs, pokemon, redirect_rs, sort_rs, quote,],
        )
        .register("/", catchers![internal_error, not_found])
}
