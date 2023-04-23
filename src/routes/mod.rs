mod err;
mod index;
pub mod json;
pub mod mongo;
mod pokemonapi;
mod random;
pub mod redirect;
mod sort;
mod sort_algs;
mod tokio_rs;
pub use err::not_found;
pub use index::index as index_rs;
pub use json::example as json_example;
pub use json::ping;
pub use json::test;
pub use json::wiktrek;
pub use pokemonapi::pokemon;
pub use random::random as random_rs;
pub use redirect::redirect as redirect_rs;
pub use sort::sort as sort_rs;
pub use tokio_rs::tokio_rs as tokio;
// pub use mongo::user_create;
pub use err::internal_error;
// pub use db::save;
// pub use db::delete;
