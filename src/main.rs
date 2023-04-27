#[macro_use]
extern crate rocket;
mod api;

mod models;
mod repository;
mod routes;
use api::*;
use models::*;
use repository::*;
use routes::*;
#[launch]
fn rocket() -> _ {
    let db = mongo_db::MongoRepo::init();
    rocket::build()
        .manage(db)
        .mount(
            "/",
            routes![
                index_rs,
                pokemon,
                redirect_rs,
                sort_rs,
                quote,
                get_user,
                create_user,
                update_user,
            ],
        )
        .register("/", catchers![internal_error, not_found])
}
