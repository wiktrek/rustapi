use rocket::*;
use rocket::serde::{json::Json, Deserialize, Serialize};
use reqwest;
use serde_json::Value;
use std::fs;
use exitfailure::ExitFailure;
#[derive(Deserialize, Serialize)]
pub struct RedirectData {
    name: String,
    redirect_url: String,
    status: String,
}
#[get("/redirect/<link>")]
pub async fn redirect(link: String) -> Result<String, ExitFailure>{
    let body = reqwest::get(format!("http://127.0.0.1:8000/data/redirect/{}", link))
    .await?
    .text()
    .await?;
println!("{:?}", body);
Ok("ez".to_string())
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


