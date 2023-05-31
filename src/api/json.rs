use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::*;
#[derive(Deserialize, Serialize)]
pub struct Response {
    response: String,
    status: String,
}
#[get("/api/wiktrek")]
pub fn wiktrek() -> Json<Response> {
    let response = Response {
        response: "wiktrek".to_string(),
        status: "200".to_string(),
    };
    return Json(response);
}
#[get("/api/test")]
pub fn test() -> Json<Response> {
    let response = Response {
        response: "api works!".to_string(),
        status: "200".to_string(),
    };
    return Json(response);
}
