use rocket::serde::json::Json;
use super::Ping;
fn swap(mut v: Vec<i32>, x: usize, y: usize) -> Vec<i32>{
  let temp = v[x];
  v[x] = v[y];
  v[y] = temp;
return v;
}
fn bubble(mut v: Vec<i32>, n: usize) {
     let i = n-1;
     while i>0 {
        if v[i]<v[i-1] {
    v = swap(v, i, i-1)
        }
     }
}
pub fn bubble_sort(mut v: Vec<i32>) -> Json<Ping> {
    let n = v.len();
    let mut i = 0;
while i < n-1 {
    bubble(v.clone(), n)
}

    Json(Ping {
    response: format!("sorted: {:?}", v),
    status: "200".to_string()
  })
}