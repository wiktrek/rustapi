use rocket::serde::json::Json;
use rocket::*;

use super::sort_algs;
use sort_algs::*;
#[get("/sort/<alg>/<data>")]
pub fn sort(alg: String, data: String) -> Json<Ping> {
    let vec: Vec<&str> = data.split(",").filter(|s| !s.is_empty()).collect();
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
        "selection" => selection_sort(vec_i32),
        "bubble" => bubble_sort(vec_i32),
        "insertion" => insertion_sort(vec_i32),
        "merge" => merge_sort(vec_i32),
        _ => {
            return Json(Ping {
                response: format!("Error: couldn't find sorting algorithm {}", algstr),
                status: "400".to_string(),
            })
        }
    }
}
