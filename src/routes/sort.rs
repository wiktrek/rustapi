use rocket::*;
use rocket::serde::{json::Json, Deserialize, Serialize};
#[derive(Deserialize, Serialize)]
pub struct Ping {
    response: String,
    status: String,
}
#[get("/sort/<data>")]
pub fn sort(data: String) -> Json<Ping>{
  let vec: Vec<&str> = data.split(",").filter(|s| !s.is_empty())
    .collect();
    let mut sorted: Vec<i32> = vec![];
    for i in vec.iter() {
    let num = i.parse::<i32>().unwrap(); 
    sorted.push(num);
    }
    println!("{:?}", vec);
    let response = Ping {
    response: format!("{:?}",sorted),
    status: "200".to_string(),
};
Json(response)
}