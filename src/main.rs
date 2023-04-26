#[macro_use]
extern crate rocket;
mod routes;
use routes::*;
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/",
            routes![
                index_rs,
                pokemon,
                ping,
                test,
                redirect_rs,
                wiktrek,
                json_example,
                sort_rs,
                tokio,
                quote,
                
                random_rs
            ],
        )
        .register("/", catchers![internal_error, not_found])
}
