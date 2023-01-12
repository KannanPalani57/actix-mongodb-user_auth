use crate::{models::user_model::User, repository::mongodb_repo::MongoRepo};
use actix_web::{
    post,
    web::{ Data, Json },
    HttpResponse,
};

#[post["/user"]]
pub async fn create_user(db: Data<MongoRepo>, new_user: Json<User>) ->  HttpResponse {
    let data =  User{
        id: None,
        first_name: new_user.first_name.to_owned(),
        last_name: new_user.last_name.to_owned(),
        username: new_user.username.to_owned(),
        age: new_user.age,
    };
    
    let user_detail = db.create_user(data).await;

    match user_detail {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
