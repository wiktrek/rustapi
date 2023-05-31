use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::*;

#[derive(Deserialize, Serialize)]
pub struct Link {
    name: String,
    link: String,
}
#[derive(Deserialize, Serialize)]
pub struct Response {
    response: String,
    status: String,
}

#[get("/api/mylinks")]
pub fn mylinks() -> Json<Response> {
    let mylinks: Vec<Link> = vec![Link {
        name: "yt".to_string(),
        link: "https://www.youtube.com/@wiktrek".to_string(),
    }];

    return Json(Response {
        response: format!("{:?}", mylinks),
        status: "400".to_string(),
    });
}
