use rocket::serde::{Deserialize, Serialize};
pub mod selection;
pub mod rust;
pub mod simple;
pub use rust::rust_sort;
pub use selection::selection_sort as selection_sort;
pub use simple::simple_sort;
#[derive(Deserialize, Serialize)]
pub struct Ping {
    pub response: String,
    pub status: String,
} 