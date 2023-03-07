use rocket::*;
use dotenv;
use ::serde::{Deserialize, Serialize};
use mongodb::{ Client,bson::doc};
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
    let handle = tokio::runtime::Handle::try_current().unwrap();
    handle.spawn(save(data.to_string(), title.to_string()));
        return "save"
    },
    "delete"=> {
     let handle = tokio::runtime::Handle::try_current().unwrap();
    handle.spawn(delete(data.to_string(), title.to_string()));
        return "delete"
    },
_ => return "choose one option save or delete"
}
}

async fn save(data: String, title: String) -> Result<(),  ExitFailure> {
let client = Client::with_uri_str(dotenv::var("DB").unwrap()).await?;
let database = client.database("rust");
let collection = database.collection::<File>("files");
println!("{:?}", collection);
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
println!("zapisano??");
println!("{}", dotenv::var("DB").unwrap());

Ok(())
}
async fn delete(data: String, title: String) -> Result<(),  ExitFailure>{
let client = Client::with_uri_str(dotenv::var("DB").unwrap()).await?;
let database = client.database("rust");
let collection = database.collection::<File>("files");
println!("{:?}", collection);
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
println!("zapisano??");
println!("{}", dotenv::var("DB").unwrap());

Ok(())
}