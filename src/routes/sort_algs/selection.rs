use rocket::serde::{json::Json, Deserialize, Serialize};
#[derive(Deserialize, Serialize)]
pub struct Ping {
    response: String,
    status: String,
}
fn loc_of_smallest(v: Vec<i32>, mut i: usize, n: usize) -> usize{
let mut j = i;
while i<=n {
  if v[i] < v[j] {
    j = i;
  }
  i += 1;
}
return j
}
fn swap(mut v: Vec<i32>, x: usize, y: usize) -> Vec<i32>{
  let temp = v[x];
  v[x] = v[y];
  v[y] = temp;
return v;
}
pub fn selection_sort(mut v: Vec<i32>) -> Json<Ping> {
    let mut i = 0;
    let n: usize = v.len();
    while i<n-1 {
      let j = loc_of_smallest(v.clone(), i, n-1);
      v = swap(v.clone(),i,j);
      i +=1
    }

    Json(Ping {
    response: format!("sorted: {:?}", v),
    status: "200".to_string()
  })
}