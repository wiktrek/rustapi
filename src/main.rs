#[macro_use]
extern crate rocket;

mod api;
mod err;
mod routes;
use api::*;
use err::*;
use routes::*;
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/",
            routes![index, wiktrek, test, pokemon_name, chucknorris, hello],
        )
        .register("/", catchers![internal_error, not_found])
}
