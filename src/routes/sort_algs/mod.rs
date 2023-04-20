use rocket::serde::{Deserialize, Serialize};
mod selection;
mod rust;
mod simple;
mod merge;
mod bubble;
mod insertion;
pub use bubble::bubble_sort;
pub use insertion::insertion_sort;
pub use merge::merge_sort;
pub use rust::rust_sort;
pub use selection::selection_sort as selection_sort;
pub use simple::simple_sort;
#[derive(Deserialize, Serialize)]
pub struct Ping {
    pub response: String,
    pub status: String,
} 