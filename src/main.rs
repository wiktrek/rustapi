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
            routes![
                index,
                htmx,
                ping,
                test,
                pokemon_name,
                chucknorris,
                hello,
                redirect_to,
                hot_take,
            ],
        )
        .register("/", catchers![internal_error, not_found])
}
