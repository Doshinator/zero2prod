//! src/routes/health_check.rs
use actix_web::{get, HttpResponse, Responder};

#[get("/health_check")]
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}