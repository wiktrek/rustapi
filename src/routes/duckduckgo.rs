use rocket::*;
use rocket::response::Redirect;
#[get("/duckduckgo/<q>")]
pub async fn duckduckgo(q: String) -> Result<Redirect, std::io::Error> {
let redirect_url = format!("https://duckduckgo.com/?q={}", q);
Ok(Redirect::to(redirect_url))
}