use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::*;
use std::io;
#[derive(Deserialize, Serialize)]
pub struct Response {
    response: String,
    status: String,
}
#[get("/api/pokemon/<name>")]
pub async fn pokemon_name(name: &str) -> io::Result<Json<Response>> {
    // https://pokeapi.co/api/v2/pokemon/pokemonname
    println!("{}", name);
    let client = reqwest::Client::new();
    let response = client
        .get(format!("https://pokeapi.co/api/v2/pokemon/{}", name))
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .expect(" Couldn't find pokemon");
    let api_response = Response {
        response,
        status: "200".to_string(),
    };
    Ok(Json(api_response))
}
