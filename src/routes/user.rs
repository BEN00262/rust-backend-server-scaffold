use actix_web::{web, post, get, HttpResponse, Scope};

use futures::stream::TryStreamExt;
use mongodb::{Collection, Database};

use crate::models::user::{
    User
};

const COLLECTION_NAME:&str = "users";

#[get("/all")]
async fn user_handler(client: web::Data<Database>) -> HttpResponse {
    let collection:Collection<User> = client.collection(COLLECTION_NAME);
    match collection.find(None, None).await {
        Ok(mut cursor) => {
            let mut users: Vec<User> = vec![];

            while let Ok(Some(user)) = cursor.try_next().await {
                users.push(user)
            }

            return HttpResponse::Ok().json(users)
        },
        Err(e) => return HttpResponse::InternalServerError().json(e.to_string()),
    };
}

#[post("/user")]
async fn create_user(client: web::Data<Database>, user: web::Json<User>) -> HttpResponse {
    let collection = client.collection(COLLECTION_NAME);
    let result = collection.insert_one(user.into_inner(), None).await;

    match result {
        Ok(_) => HttpResponse::Ok().body("User created"),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

pub fn user_routes() -> Scope {
    web::scope("/user")
        .service(user_handler)
        .service(create_user)
}