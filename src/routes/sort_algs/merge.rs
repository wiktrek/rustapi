use rocket::serde::json::Json;
use super::Ping;
pub fn merge_sort(mut v: Vec<i32>) -> Json<Ping> {
  // https://www.youtube.com/watch?v=l7-f9gS8VOs
    Json(Ping {
    response: format!("response"),
    status: "200".to_string()
  })
}