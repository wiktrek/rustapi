use rocket::*;
use rocket::response::Redirect;
use rocket::serde::{json::Json, Deserialize, Serialize};
use reqwest::{self, get};
use serde_json::Value;
use std::fs;
#[derive(Deserialize, Serialize)]
pub struct RedirectData {
    name: String,
    redirect_url: String,
    status: String,
}

#[get("/redirect/<link>")]
pub async fn redirect(link: String) -> Redirect{
    let redirect_uri= uri!("https://wiktrek.xyz");
let handle = tokio::runtime::Handle::try_current().unwrap();
let result = handle.block_on(get_url(link));
println!("{:?}", result);
Redirect::to(redirect_uri)
}


async fn get_url(link: String) -> Result<String, Box<dyn std::error::Error>>{

let body = reqwest::get(format!("http://127.0.0.1:8000/data/redirect/{}", link)).await?.text().await?;
println!("{:?}", body);
Ok("eee".to_string())
}

#[get("/data/redirect/<link>")]
pub fn redirect_data(link: String) -> Json<RedirectData>{

let response = RedirectData {
    name: "".to_string(),
    redirect_url: "https://example.com".to_string(),
    status: "200".to_string(),
};
Json(response)
}


