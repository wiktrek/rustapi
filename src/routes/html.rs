use rocket::response::content::RawHtml;
use rocket::*;
#[get("/")]
pub fn index() -> RawHtml<&'static str> {
    RawHtml(include_str!("./html/index.html"))
}
#[get("/htmx")]
pub fn htmx() -> RawHtml<&'static str> {
    RawHtml(include_str!("./html/htmx.html"))
}
