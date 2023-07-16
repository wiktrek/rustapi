use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::*;
#[derive(Deserialize, Serialize)]
pub struct Response {
    response: String,
    status: i32,
}
#[get("/api/wiktrek")]
pub fn wiktrek() -> Json<Response> {
    let response = Response {
        response: "wiktrek".to_string(),
        status: 200,
    };
    Json(response)
}
#[get("/api/test")]
pub fn test() -> Json<Response> {
    let response = Response {
        response: "api works!".to_string(),
        status: 200,
    };
    Json(response)
}
#[get("/api/hello/<name>/<age>")]
pub fn hello(name: &str, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}
