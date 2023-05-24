#[macro_use]
extern crate rocket;
mod err;
mod routes;
use err::*;
use routes::*;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index_rs])
        .register("/", catchers![internal_error, not_found])
}
