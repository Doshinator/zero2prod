//! src/routes/name.rs
use actix_web::{post, HttpResponse, Responder};

// // to test post enpoint, in terminal
// // curl -X POST 127.0.0.1:8080/name -H "Content-Type: plain/text" -d "request body here"
#[post("/name")]
pub async fn name(name: String) -> impl Responder {
    HttpResponse::Ok().body(format!("Welcome {}", name))
}