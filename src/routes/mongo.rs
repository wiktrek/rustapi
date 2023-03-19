    use std::env;
    extern crate dotenv;
    use dotenv::dotenv;

    use mongodb::{
        bson::{extjson::de::Error},
        results::{ InsertOneResult},
        sync::{Client, Collection},
    };

    pub struct MongoRepo {
        col: Collection<User>,
    }
pub struct User {
    id: String,
    username: String,
    password: String,

}
    impl MongoRepo {
        pub fn init() -> Self {
            dotenv().ok();
            let uri = match env::var("DB") {
                Ok(v) => v.to_string(),
                Err(_) => format!("Error loading env variable"),
            };
            let client = Client::with_uri_str(uri).unwrap();
            let db = client.database("rust");
            let col: Collection<User> = db.collection("User");
            MongoRepo { col }
        }

        pub fn create_user(&self, new_user: User) -> Result<InsertOneResult, Error> {
            let new_doc = User {
                id: new_user.id,
                username: new_user.username,
                password: new_user.password,
            };
            let user = self
                .col
                .insert_one(new_doc, None)
                .ok()
                .expect("Error creating user");
            Ok(user)
        }
    fn get_collection(&self) -> Collection {
    
        self.col
    }
        pub async fn get_users(&self) -> Result<Vec<User>> {
        let mut cursor = self
            .get_collection()
            .find(None, None)
            .await
            .map_err(MongoQueryError)?;

        let mut result: Vec<User> = Vec::new();
        while let Some(doc) = cursor.next().await {
            result.push(self.doc_to_users(&doc?)?);
        }
        Ok(result)
    }
    pub fn doc_to_users(&self, doc: &Document) -> Result<User>{
let user = User {
    id: doc.get_str(ID),
    username: doc.get_str(USERNAME),
    password: doc.get_str(PASSWORD)
};
Ok(user)
    }

    }