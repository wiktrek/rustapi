use rocket::*;
use rocket::serde::json::Json;
#[get("/ping")]
pub fn ping() -> Json<String>{
Json("pong".to_string())
}

