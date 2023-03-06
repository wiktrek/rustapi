use rocket::*;
use dotenv;
use mongodb::{Client, options::ClientOptions};
#[get("/db/<option>/<data>")]
pub fn db<'a>(option: &str, data: &str) -> &'a str {
match option {
    "save"=> {
        save(data);
        return "save"
    },
    "delete"=> {
        delete(data);
   return "delete"
    },
_ => return "choose one option save or delete"
}
}
fn save(data: &str) {
// let mut client_options = ClientOptions::parse().await?;
println!("save");
println!("{}",  dotenv::var("DB").unwrap());


}
fn delete(data: &str) {

}