use rocket::serde::json::Json;
use super::Ping;
pub fn simple_sort(vec: Vec<i32>) -> Json<Ping> {
  let mut sorted = vec;
  let n = sorted.len();
let mut i: usize = 0;
  while i < n {
  let mut j = i + 1;
    while j <n {  
      if sorted[j]<sorted[i] {
        let temp = sorted[i];
        sorted[i] = sorted[j];
        sorted[j] = temp;
      }
    j+= 1;
    }
  i += 1;
  }
  Json(Ping {
    response: format!("{:?}", sorted),
    status: "200".to_string()
  })
}