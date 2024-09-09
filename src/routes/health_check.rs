//! src/routes/health_check.rs
use actix_web::{get, HttpResponse, Responder};

#[tracing::instrument(name = "Health Check")]
#[get("/health_check")]
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}
