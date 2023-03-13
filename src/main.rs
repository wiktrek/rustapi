#[macro_use]
extern crate rocket;

mod routes;
use routes::*;
#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index_rs, ping, test, redirect_rs, wiktrek, json_example])
}
