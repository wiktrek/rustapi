use rand::Rng;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::*;
use std::fs;

#[derive(Deserialize, Serialize)]
pub struct Response {
    take: String,
    author: String,
    status: i32,
}
#[derive(Deserialize, Serialize)]
struct HotTake {
    take: String,
    author: String,
}
#[get("/api/hottake")]
pub async fn hot_take() -> Json<Response> {
    let file = fs::read_to_string("./src/data/hottake.json").unwrap();
    let takes: Vec<HotTake> = serde_json::from_str(&file).expect("Error parsing json");

    let take = &takes[0];
    let api_response = Response {
        take: take.take.to_string(),
        author: take.author.to_string(),
        status: 200,
    };

    Json(api_response)
}
