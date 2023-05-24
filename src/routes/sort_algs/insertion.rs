use rocket::serde::json::Json;
use super::Ping;
pub fn insertion_sort(mut v: Vec<i32>) -> Json<Ping> {
    for i in 1..v.len() {
        let mut j = i;
        while j > 0 && v[j - 1] > v[j] {
            v.swap(j - 1, j);
            j -= 1;
        }
    }
     Json(Ping {
    response: format!("sorted: {:?} ", v),
    status: "200".to_string()
  })   
}