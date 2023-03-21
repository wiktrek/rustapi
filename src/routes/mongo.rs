#![allow(unused)]
use std::env;
use bson::doc;
use dotenv::dotenv;
use mongodb::{results::InsertOneResult,Client,options::ClientOptions};
use rocket::*;
use rocket::http::Status;
use rocket::serde::{json::Json, Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct User{
    username: String,
    password: String,
    id: String,
}
#[derive(Deserialize, Serialize)]
pub struct Response{
    response: String,
    status: String,
}
struct Err {}
impl From<mongodb::error::Error> for Err {
    fn from(_error: mongodb::error::Error) -> Self {
        Err {}
    }
}

#[get("/user/create/<username>/<password>")]
pub async fn user_create(username: String, password: String) -> Result<Json<InsertOneResult>, mongodb::error::Error> {
     dotenv().ok();
                 let uri = match env::var("DB") {
                Ok(v) => v.to_string(),
                Err(_) => format!("Error loading env variable"),
            };
                    let client_options = ClientOptions::parse(uri).await?;
                let client = Client::with_options(client_options).await;
            println!("client: {:?}", client);
            let database = client.database("rust");
            println!("----------------------------");
            println!("database: {:?}", database);
            let collection = database.collection::<User>("users");
    let user = User {
        username,
        password,
        id: "wiktrek".to_string(),
};
    let insert = collection.insert_one(user, None).await.ok().expect("Error creating user");
    let response = Response {
    response: "DATA WRITTEN to".to_string(),
    status: "200".to_string(),
};


Ok(Json(insert))
}