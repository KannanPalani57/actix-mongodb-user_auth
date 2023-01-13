use std::env;
extern crate dotenv;
use dotenv::dotenv;

use mongodb::{
    bson::{extjson::de::Error, oid::ObjectId, doc} ,
    results::{ InsertOneResult, UpdateResult},
    Client, Collection
};

use crate::models::user_model::User;

pub struct MongoRepo {
    col: Collection<User>,
}

impl MongoRepo {
    pub async fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGO_URI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error on loading env variable"),
        };
        let client = Client::with_uri_str(uri).await.unwrap();
        let db = client.database("rustWithUser");
        let col: Collection<User> = db.collection("User");

        MongoRepo { col }
    }

    pub async fn create_user(&self, new_user: User) -> Result<InsertOneResult, Error> {
        let new_doc = User {
            id: None,
            first_name: new_user.first_name,
            last_name : new_user.last_name,
            username: new_user.username,
            age: new_user.age,
        };

        let user = self.col.insert_one(new_doc, None)
            .await.ok().expect("Error on creating the User");
        
        Ok(user)
    }

    pub async fn get_user(&self, id: &String) -> Result<User, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! { "_id": obj_id };
        let user_detail = self.col.find_one(filter, None)
            .await.ok().expect("Error getting user's details");

        Ok(user_detail.unwrap())
    }

    pub async fn update_user(&self, id: &String, new_user: User) -> Result<UpdateResult, Error>{
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! { "_id": obj_id };

        let new_doc = doc! {
            "$set":{
            "first_name": new_user.first_name,
            "last_name": new_user.last_name,
            "username": new_user.username,
            "age": new_user.age
        }};
        println!("obj Id {:?}", filter);
        println!("{:?}", new_doc);
        let update_doc = self.col.update_one(filter, new_doc, None).await.ok()
            .expect("Error updating user");

        Ok(update_doc)
    }
}