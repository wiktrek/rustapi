use rocket::serde::json::Json;
use super::Ping;
pub fn rust_sort(vec: Vec<i32>) -> Json<Ping>{
  let mut sorted = vec;
    sorted.sort();
    let response = Ping {
    response: format!("{:?}", sorted),
    status: "200".to_string(),
  };
Json(response)
}