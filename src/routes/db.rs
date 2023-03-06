use rocket::*;
use dotenv;
use mongodb::{Client, options::ClientOptions};
#[get("/db/<option>/<data>")]
pub fn db<'a>(option: &str, data: &str) -> &'a str {
match option {
    "save"=> {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let future = save(data);
    rt.block_on(future);
        return "save"
    },
    "delete"=> {
        delete(data);
   return "delete"
    },
_ => return "choose one option save or delete"
}
}
async fn save(data: &str) {
let mut client_options = ClientOptions::parse(dotenv::var("DB").unwrap()).await?;
println!("save");
println!("{}", dotenv::var("DB").unwrap());


}
fn delete(data: &str) {

}