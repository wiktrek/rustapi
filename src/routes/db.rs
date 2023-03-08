use rocket::*;
use dotenv;
use ::serde::{Deserialize, Serialize};
use mongodb::{ Client,bson::doc};
use exitfailure::ExitFailure;
#[derive(Debug, Serialize, Deserialize)]
struct Files {
    owner: String,
    title: String,
    data: String,
    id: String,
}

// pub fn db<'a>(option: &str, title: &str, data: &str) -> &'a str {
//     println!("{}, {}, {}", option, title, data);
// match option {
//     "save"=> {
//     let handle = tokio::runtime::Handle::try_current().unwrap();
//     handle.spawn(save(data.to_string(), title.to_string()));
//         return "save"
//     },
//     "delete"=> {
//      let handle = tokio::runtime::Handle::try_current().unwrap();
//     handle.spawn(delete(data.to_string(), title.to_string()));
//         return "delete"
//     },
// _ => return "choose one option save or delete"
// }
// }
#[get("/db/save/<username>/<id>/<title>/<data>")]
pub async fn save<'r,'a>(username: &str, id: &str, title: &str, data: &str) -> Result<(),  ExitFailure> {
let client = Client::with_uri_str(dotenv::var("DB").unwrap()).await?;
let database = client.database("rust");
let collection = database.collection::<Files>("files");
println!("{:?}", collection);
let file = Files {
    owner: username.to_string(),
    title: title.to_string(),
    data: data.to_string(),
    id: id.to_string(),
};
collection.insert_one(file, None);
// let cursor = collection.find(doc! { "title": title.to_string()}, None).await?;
// for result in cursor {
//     println!("title: {}", result?.title);
// }
println!("zapisano??");
println!("{}", dotenv::var("DB").unwrap());

Ok(())
}
#[get("/db/delete/<username>/<id>/<title>/<data>")]
pub async fn delete(username: &str, id: &str, title: &str, data: &str) -> Result<(), ExitFailure>{
let client = Client::with_uri_str(dotenv::var("DB").unwrap()).await?;
let database = client.database("rust");
let collection = database.collection::<Files>("files");
let file = doc! {
    "owner": username.to_string(),
    "title": title.to_string(),
    "data": data.to_string(),
    "id": id.to_string(),
    "id": id.to_string(),
};
collection.delete_one(file, None).await?;
// let cursor = collection.find(doc! { "title": title.to_string()}, None).await?;
// for result in cursor {
//     println!("title: {}", result?.title);
// }
println!("zapisano??");
println!("{}", dotenv::var("DB").unwrap());

Ok(())
}