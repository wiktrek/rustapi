mod index;
pub mod db;
pub use index::index as index_rs;
pub use db::save;
pub use db::delete;