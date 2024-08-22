//! src/routes/greeting.rs
use actix_web::{get, HttpResponse, Responder};

#[get("/greeting")]
pub async fn greeting() -> impl Responder {
    HttpResponse::Ok().body("Greeting!")
}