pub mod selection;
pub mod rust;
pub mod simple;
pub use rust::rust_sort;
pub use selection::selection_sort as selection_sort;
pub use simple::simple_sort;