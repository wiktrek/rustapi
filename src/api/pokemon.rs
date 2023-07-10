use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::*;
use std::io;
#[derive(Deserialize, Serialize)]
pub struct Response {
    response: PokeApi,
    status: String,
}
#[derive(Deserialize, Serialize)]
struct PokeApi {
    name: String,
    height: i32,
    id: i32,
    weight: i32,
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
        .unwrap();
    let data = response
        .json::<PokeApi>()
        .await
        .expect(format!("Error: Couldn't find pokemon: {}", name).as_str());
    let api_response = Response {
        response: data,
        status: "200".to_string(),
    };
    Ok(Json(api_response))
}
