use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::*;
use std::io;
#[derive(Deserialize, Serialize)]
pub struct Response {
    response: String,
    status: i32,
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

// https://pokeapi.co/api/v2/pokemon/pokemonname
#[get("/api/pokemon/<name>")]
pub async fn pokemon_name(name: &str) -> io::Result<Json<Response>> {
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
        status: 200,
    };
    Ok(Json(api_response))
}

// https://api.chucknorris.io/jokes/random
#[get("/api/chucknorris")]
pub async fn chucknorris() -> Json<Response> {
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
        status: 200,
    };
    Json(api_response)
}
