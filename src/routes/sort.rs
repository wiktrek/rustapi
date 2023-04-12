use rocket::*;
use rocket::serde::{json::Json, Deserialize, Serialize};
#[derive(Deserialize, Serialize)]
pub struct Ping {
    response: String,
    status: String,
}
#[get("/sort/<data>")]
pub fn sort(data: String) -> Json<Ping>{
  let mut vec: Vec<&str> = data.split(",").filter(|s| !s.is_empty())
    .collect();
    println!("{:?}", vec);
    let response = Ping {
    response: format!("{:?}",vec.sort()),
    status: "200".to_string(),
};
Json(response)
}