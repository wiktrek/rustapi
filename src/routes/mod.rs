mod index;
pub mod db;
pub mod json;
pub mod redirect;
pub use index::index as index_rs;
pub use json::ping;
pub use json::test;
pub use json::wiktrek;

pub use redirect::redirect as redirect_rs;
// pub use db::save;
// pub use db::delete;