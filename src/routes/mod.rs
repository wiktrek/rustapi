mod err;
mod index;
mod pokemonapi;
mod quoteapi;
pub mod redirect;
mod sort;
mod sort_algs;

pub use err::internal_error;
pub use err::not_found;
pub use index::index as index_rs;
pub use pokemonapi::pokemon;
pub use quoteapi::quote;
pub use redirect::redirect as redirect_rs;
pub use sort::sort as sort_rs;
// pub use db::save;
// pub use db::delete;
