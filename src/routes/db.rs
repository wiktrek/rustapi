use rocket::*;
use dotenv;
use ::serde::{Deserialize, Serialize};
use mongodb::{ Client,bson::doc};
use exitfailure::ExitFailure;
#[derive(Debug, Serialize, Deserialize)]
struct Files {
    title: String,
    data: String,
    id: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct User {
    name: String,
    password: String,
    id: String,
    files: Vec<Files>
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
#[get("/db/save/<username>/<password>/<id>/<title>/<data>")]
pub async fn save<'r,'a>(username: &str, password: &str, id: &str, title: &str, data: &str) -> Result<(),  ExitFailure> {
let client = Client::with_uri_str(dotenv::var("DB").unwrap()).await?;
let database = client.database("rust");
let collection = database.collection::<User>("files");
println!("{:?}", collection);
let user = User {
    name: username.to_string(),
    password: password.to_string(),
    id: id.to_string(),
    files: vec![Files {
    title: title.to_string(),
    data: data.to_string(),
    id: id.to_string()

    }]
};
collection.insert_one(user, None);
// let cursor = collection.find(doc! { "title": title.to_string()}, None).await?;
// for result in cursor {
//     println!("title: {}", result?.title);
// }
println!("zapisano??");
println!("{}", dotenv::var("DB").unwrap());

Ok(())
}
#[get("/db/delete/<id>/<fileid>")]
pub async fn delete(id: &str, fileid: &str) -> Result<(),  ExitFailure>{
let client = Client::with_uri_str(dotenv::var("DB").unwrap()).await?;
let database = client.database("rust");
let collection = database.collection::<User>("files");
collection.delete_one().await?;
// let cursor = collection.find(doc! { "title": title.to_string()}, None).await?;
// for result in cursor {
//     println!("title: {}", result?.title);
// }
println!("zapisano??");
println!("{}", dotenv::var("DB").unwrap());

Ok(())
}