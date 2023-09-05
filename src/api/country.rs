use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::*;
#[derive(Deserialize, Serialize)]
struct Name {
    common: String,
    official: String,
}
#[derive(Deserialize, Serialize)]
struct Country {
    name: Name,
    tld: Vec<String>,
    flags: Vec<String>,
}
#[derive(Deserialize, Serialize)]
pub struct Response {
    response: String,
    status: i32,
}
// https://restcountries.com/v3.1/name/poland
#[get("/api/countries/<name>")]
pub async fn countries(name: &str) -> Json<Response> {
    let client = reqwest::Client::new();
    let response = client
        .get(format!("https://restcountries.com/v3.1/name/{}", name))
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .send()
        .await
        .unwrap();
    let data = response.json::<Country>().await.unwrap();
    let api_response = Response {
        response: data.name.common,
        status: 200,
    };
    Json(api_response)
}
