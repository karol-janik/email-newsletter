use actix_web::{web, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct User {
    Name: String,
    Email: String,
}

pub async fn subscribe(_user: web::Form<User>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
