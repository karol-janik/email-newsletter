use actix_web::{web, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct User {
    name: String,
    email: String,
}

pub async fn subscribe(_user: web::Form<User>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
