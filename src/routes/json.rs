use rocket::*;
use rocket::serde::{json::Json, Deserialize, Serialize};
#[get("/ping")]
pub fn ping() -> Json<String>{
Json("pong".to_string())
}
#[derive(Deserialize, Serialize)]
pub struct Test {
 id: String,
 status: String,
}
#[get("/test")]
pub fn test() -> Json<Test>{
let response = Test {
    id: "test".to_string(),
    status: "200".to_string(),
};


Json(response)
}

