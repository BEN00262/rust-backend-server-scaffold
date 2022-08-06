#[macro_use]
extern crate dotenv_codegen;

use actix_web::{web, App, HttpServer};
use mongodb::{Client};

mod models;
mod routes;

use routes::user as UserRoutes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client = Client::with_uri_str(dotenv!("MONGODB_URI")).await.expect("Failed to initialize DB client");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.database(dotenv!("DATABASE"))))
            .service(
                UserRoutes::user_routes()
            )
    })
    .bind("localhost:8080")?
    .run()
    .await
}