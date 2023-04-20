use rocket::*;
use rocket::serde::{json::Json, Deserialize, Serialize};
#[derive(Deserialize, Serialize)]
pub struct Ping {
    response: String,
    status: String,
}
#[get("/sort/<alg>/<data>")]
pub fn sort(alg: String, data: String) -> Json<Ping>{
  let vec: Vec<&str> = data.split(",").filter(|s| !s.is_empty())
    .collect();
  let algstr = alg.as_str();


    println!("{:?}", vec);
match algstr {
  "rust" => rust_sort(vec),
  _ => return Json(Ping { 
    response: format!("Error: couldn't find sorting algorithm {}", algstr),
    status: "400".to_string()}) 
}
}
fn rust_sort(vec: Vec<&str>) -> Json<Ping>{
    let mut sorted: Vec<i32> = vec![];
    for i in vec.iter() {
    let num = i.parse::<i32>().unwrap(); 
    sorted.push(num);
    }
    sorted.sort();
    let response = Ping {
    response: format!("{:?}", sorted),
    status: "200".to_string(),
  };
Json(response)
}