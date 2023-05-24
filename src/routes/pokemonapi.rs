use reqwest::*;
// use rocket::serde::json::to_string;
use std::{io::Error, result::Result};
#[get("/pokemon/<option>/<pokemon>")]
pub async fn pokemon(option: String, pokemon: String) -> Result<String, Error> {
    let url = format!("https://pokeapi.co/api/v2/pokemon/{}", pokemon);
    let response = get_response(url, option).await.unwrap();

    Ok(response)
}
async fn get_response(url: String, option: String) -> Result<String, Error> {
    let response = get(url).await.unwrap().text().await;
    println!("{:?}", response);
    match option.as_str() {
        "height" => height(),
        _ => return Ok("WRONG OPTION".to_string()),
    }
    Ok("response".to_string())
}
fn height() {
    // get height from api
}
