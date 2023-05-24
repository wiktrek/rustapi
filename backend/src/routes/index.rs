use rocket::response::content::RawHtml;
#[get("/")]
pub fn index_rs() -> RawHtml<&'static str> {
    RawHtml(include_str!("../html/index.html"))
}
