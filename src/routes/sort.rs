// use rocket::*;
// use rocket::serde::{json::Json, Deserialize, Serialize};
// #[derive(Deserialize, Serialize)]
// pub struct Ping {
//     response: String,
//     status: String,
// }
// #[get("/sort/<data>")]
// pub fn sort(data: String) -> Json<Ping>{
//   let vec: Vec<&str> = data.split(",").filter(|s| !s.is_empty())
//     .collect();
//     println!("{:?}", vec);
//     let mut biggest:i32;
//     let mut smallest:i32;
//     let mut last:i32 = 0;
//     let mut r:Vec<i32>;
//     for i in vec.iter() {
//     println!("{:?}", i);
//     let num = i.parse::<i32>().unwrap();
//     if num < smallest {
//         smallest = num;
//     }


//     last = num
//     }
//     let response = Ping {
//     response: format!("{:?}",vec),
//     status: "200".to_string(),
// };
// Json(response)
// }