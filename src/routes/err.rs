use rocket::*;

#[catch(404)]
pub fn not_found(req: &Request) -> String {
    format!("I couldn't find '{}'. Try something else?", req.uri())
}
#[catch(500)]
pub fn internal_error() -> &'static str {
    "There was an error.Try again later."
}
