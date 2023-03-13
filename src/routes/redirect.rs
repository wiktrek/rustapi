use rocket::*;
use rocket::response::Redirect;
use rocket::serde::{json::Json, Deserialize, Serialize};
use reqwest::{self, get};
use serde_json::Value;
use std::fs;
#[derive(Deserialize, Serialize, Debug)]
struct Links {
    name: String,
    redirects: String,
}
#[get("/redirect/<link>")]
pub async fn redirect(link: String) -> Result<String, std::io::Error> {
    let redirect_uri= uri!("https://wiktrek.xyz");
    let result = fs::read_to_string("src/data/redirect.json")?;
println!("{:?}", result);
// Redirect::to(redirect_uri)
Ok(format!("{:?}", serde_json::from_str::<Links>(result.as_str()).unwrap()))
}