use rocket::*;
use dotenv;
use ::serde::{Deserialize, Serialize};
use mongodb::{ Client,bson::doc,options::ClientOptions};
use exitfailure::ExitFailure;
#[derive(Debug, Serialize, Deserialize)]
struct File {
    title: String,
    data: String,
}
#[get("/db/<option>/<title>/<data>")]
pub fn db<'a>(option: &str, title: &str, data: &str) -> &'a str {
    println!("{}, {}, {}", option, title, data);
match option {
    "save"=> {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let future = save(data.to_string(), title.to_string());
    rt.spawn(future);
        return "save"
    },
    "delete"=> {
        delete(data);
   return "delete"
    },
_ => return "choose one option save or delete"
}
}
async fn save(data: String, title: String) -> Result<(),  ExitFailure> {
let client = Client::with_uri_str(dotenv::var("DB").unwrap()).await?;
let database = client.database("rust");
let collection = database.collection::<File>("files");
let docs = vec![
    File{
        title: title.to_string(),
        data: data.to_string(),
    }
];
collection.insert_many(docs, None).await?;
// let cursor = collection.find(doc! { "title": title.to_string()}, None).await?;
// for result in cursor {
//     println!("title: {}", result?.title);
// }
println!("save");
println!("{}", dotenv::var("DB").unwrap());

Ok(())
}
fn delete(data: &str) {

}