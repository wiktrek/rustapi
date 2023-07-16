use rocket::response::Redirect;
use rocket::serde::{Deserialize, Serialize};
use rocket::*;

#[derive(Deserialize, Serialize)]
struct Link {
    name: String,
    redirect: String,
}

#[derive(Deserialize, Serialize)]
struct Data {
    data: Vec<Link>,
}
#[get("/redirect/<link>")]
pub async fn redirect_to(link: &str) -> Redirect {
    let get_links =
        reqwest::get("https://raw.githubusercontent.com/wiktrek/wiktrekxyz/main/data/data.json")
            .await
            .expect("Failed to get data.json")
            .json::<Data>()
            .await
            .expect("Failed to get data.json")
            .data;
    for l in get_links {
        println!("{}, {}", l.name, l.redirect);
        if link == l.name {
            return Redirect::to(l.redirect);
        }
    }

    Redirect::to(uri!("https://wiktrek.xyz"))
}
