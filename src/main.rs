mod api;
mod models;
mod repository;

use actix_web::{get, App, HttpResponse, HttpServer, Responder, web::Data};
use api::user_api::{create_user, get_user, update_user, delete_user, get_all_users};
use repository::mongodb_repo::MongoRepo;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().json("Hello from rust and mongodb")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  let db = MongoRepo::init().await;
  let db_data = Data::new(db);
  HttpServer::new(move || {
    App::new()
      .app_data(db_data.clone())
      .service(hello)
      .service(create_user)
      .service(get_user)
      .service(update_user)
      .service(delete_user)
      .service(get_all_users)
    }
    )
  .bind(("127.0.0.1", 8080))?.run().await
}

//move keyword attached to the closure gives it ownership of the Mongodb Configuration.