//! src/routes/subscriptions.rs
use actix_web::{post, web, HttpResponse, Responder};

#[allow(dead_code)]
#[derive(serde::Deserialize)]
struct FormData {
    email: String,
    name: String,
}

#[allow(dead_code)]
fn index(form: web::Form<FormData>) -> String {
    format!("Welcome {}!, Email {}", form.email, form.name)
}

#[post("/subscriptions")]
async fn subscribe(_form: web::Form<FormData>) -> impl Responder {
    HttpResponse::Ok().finish()
}

