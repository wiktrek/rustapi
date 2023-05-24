use rocket::response::Redirect;
use rocket::serde::{Deserialize, Serialize};
use rocket::*;
use std::fs;
use std::path::Path;
#[derive(Deserialize, Serialize, Debug)]
struct Links {
    name: String,
    redirect: String,
}
#[get("/redirect/<link>")]
pub async fn redirect(link: String) -> Result<Redirect, std::io::Error> {
    let redirect_uri = uri!("https://wiktrek.xyz");
    let path = Path::new("src/data/redirect.json");
    let file = fs::File::open(path).expect("failed to open");
    let read_to_json: Vec<Links> =
        serde_json::from_reader(&file).expect("error while reading or parsing");
    println!("{:?}", file);
    for l in read_to_json {
        if l.name == link {
            return Ok(Redirect::to(l.redirect));
        }
    }
    Ok(Redirect::to(redirect_uri))
}
