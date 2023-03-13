use rocket::*;
use rocket::serde::{json::Json, Deserialize, Serialize};
#[derive(Deserialize, Serialize)]
pub struct Ping {
    response: String,
    status: String,
}
#[get("/ping")]
pub fn ping() -> Json<Ping>{
    let response = Ping {
    response: "pong".to_string(),
    status: "200".to_string(),
};
Json(response)
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
#[derive(Deserialize, Serialize)]
pub struct Response {
 response: String,
 status: String,
}
#[get("/wiktrek")]
pub fn wiktrek() -> Json<Response>{
let response = Response {
    response: "wiktrek".to_string(),
    status: "200".to_string(),
};
Json(response)
}
#[get("/response/example")]
pub fn example() -> Json<Response>{
let response = Response {
    response: "response example".to_string(),
    status: "200".to_string(),
};
Json(response)
}

