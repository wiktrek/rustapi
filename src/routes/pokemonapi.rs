use reqwest::*;
use rocket::serde::json::to_string;
use std::{io::Error, result::Result};
#[get("/pokemon/<option>/<pokemon>")]
pub async fn pokemon(option: String, pokemon: String) -> Result<String, std::io::Error> {
    let url = format!("https://pokeapi.co/api/v2/pokemon/{}", pokemon);

    Ok("ez".to_string())
}
fn get_response() {
    let response = get(url).await.unwrap().text().await;
    println!("{:?}", response);
    match option.as_str() {
        "height" => height(),
        _ => return Ok("WRONG OPTION".to_string()),
    }
}
fn height() {
    // get height from api
}
