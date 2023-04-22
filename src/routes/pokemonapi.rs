use reqwest::*;
use rocket::serde::json::to_string;
use std::{io::Error, result::Result};
#[get("/pokemon/<option>/<pokemon>")]
pub async fn pokemon(option: String, pokemon: String) -> Result<String, Error> {
    let url = format!("https://pokeapi.co/api/v2/pokemon/{}", pokemon);
    let response = get(url).await.unwrap().text().await;
    println!("{:?}", response);
    Ok("ez".to_string())
}
