use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::*;
use std::io;
#[derive(Deserialize, Serialize)]
pub struct Response {
    response: String,
    status: String,
}
#[derive(Deserialize, Serialize)]
struct PokeApi {
    name: String,
    height: i32,
    id: i32,
    weight: i32,
}
#[derive(Deserialize, Serialize)]
struct ChuckNorris {
    value: String,
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
        .expect(format!("Error: Couldn't find pokemon: {}", name).as_str());
    let data = response
        .json::<PokeApi>()
        .await
        .expect(format!("Error: Couldn't find pokemon: {}", name).as_str());
    let api_response = Response {
        response: serde_json::to_string(&data).unwrap(),
        status: "200".to_string(),
    };
    Ok(Json(api_response))
}
#[get("/api/chucknorris")]
pub async fn chucknorris() -> Json<Response> {
    // https://api.chucknorris.io/jokes/random

    let client = reqwest::Client::new();
    let response = client
        .get(format!("https://api.chucknorris.io/jokes/random"))
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .send()
        .await
        .unwrap();
    let data = response.json::<ChuckNorris>().await.unwrap();
    let api_response = Response {
        response: data.value,
        status: "200".to_string(),
    };
    Json(api_response)
}
