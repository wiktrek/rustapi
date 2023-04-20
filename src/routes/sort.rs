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
  let mut vec_i32: Vec<i32> = vec![];
  let algstr = alg.as_str();
    for i in vec.iter() {
    let num = i.parse::<i32>().unwrap(); 
    vec_i32.push(num);
    }
    println!("{:?}", vec);
match algstr {
  "rust" => rust_sort(vec_i32),
  "simple" => simple_sort(vec_i32),
  _ => return Json(Ping { 
    response: format!("Error: couldn't find sorting algorithm {}", algstr),
    status: "400".to_string()})
}
}
fn simple_sort(vec: Vec<i32>) -> Json<Ping> {
  let n: i32 = vec.len() as i32;
let mut i: i32 = 0;
  while i < n {
  let mut j = i + 1;
    while j <n {
      j+= 1;
      if vec[j as usize]<vec[i as usize] {
        
      }
    }
  i += 1;
  }
  Json(Ping {
    response: format!("response"),
    status: "200".to_string()
  })
}
fn rust_sort(vec: Vec<i32>) -> Json<Ping>{
  let mut sorted = vec;
    sorted.sort();
    let response = Ping {
    response: format!("{:?}", sorted),
    status: "200".to_string(),
  };
Json(response)
}