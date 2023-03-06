
use rocket::*;
use dotenv;
use ::serde::{Deserialize, Serialize};
use mongodb::{Client, options::ClientOptions};
use exitfailure::ExitFailure;
#[derive(Debug, Serialize, Deserialize)]
struct File {
    title: String,
    data: String,
}
#[get("/db/<option>/<title>/<data>")]
pub fn db<'a>(option: &str, title: &str, data: &str) -> &'a str {
match option {
    "save"=> {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let future = save(data, title);
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
async fn save(data: &str, title: &str) -> Result<(),  ExitFailure> {
let mut client_options = ClientOptions::parse(dotenv::var("DB").unwrap()).await?;
let client = Client::with_options(client_options)?;
let database = client.database("rust");


println!("save");
println!("{}", dotenv::var("DB").unwrap());

Ok(())
}
fn delete(data: &str) {

}