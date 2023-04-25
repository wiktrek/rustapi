use reqwest::*;
// use rocket::serde::json::to_string;
use serde::{Deserialize, Serialize};
use std::{io::Error, result::Result};

#[derive(Serialize, Deserialize, Debug)]
struct Response {
    _id: String,
    author: String,
    content: String,
    tags: String,
    author_slug: String,
    length: String,
    date_added: String,
    date_modified: String,
}
#[get("/quote")]
pub async fn quote() -> Result<String, Error> {
    // i need to finish this
    let url = format!("https://api.quotable.io/random");
    let response = get(url).await.expect("error");
    // let text: Response = response.json::<Response>().await.unwrap();
    Ok(format!(
        "{}",
        response.text().await.unwrap(),
        // text.author
    ))
    // Ok(text.author)
}
