#[macro_use]
extern crate rocket;

mod routes;
use routes::*;
#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index_rs, db_rs])
}