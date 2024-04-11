use actix_web::{web, App, HttpServer,middleware::Logger};
use dotenv::dotenv;
use std::sync::Mutex;

mod routes;
mod db;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let db_client = db::init().await.expect("Failed to connect to the database");
    let db_data = web::Data::new(Mutex::new(db_client));


    HttpServer::new(move || {
        App::new()
        .app_data(db_data.clone()) 
        .wrap(Logger::default())
        .configure(routes::init_routes)
    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}