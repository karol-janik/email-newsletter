use serde::Deserialize;
use actix_web::{web, HttpResponse};

#[derive(Deserialize)]
pub struct User {
    Name: String,
    Email: String,
}

pub async fn subscribe(_user: web::Form<User>) -> HttpResponse {
    HttpResponse::Ok().finish()
}