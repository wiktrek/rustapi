use glob::glob;
pub fn loadfiles() {
    for entry in glob("./src/routes/**/*.rs").expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                println!("{:?}", path.file_name());
                println!("{:?}", path.display())
            }
            Err(e) => println!("{:?}", e),
        }
    }
}
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
