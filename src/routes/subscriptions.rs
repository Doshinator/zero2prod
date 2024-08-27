//! src/routes/subscriptions.rs
#![allow(dead_code)]
use actix_web::{post, web, HttpResponse, Responder};
use sqlx::PgPool;
use chrono::Utc; 
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

#[allow(dead_code)]
fn index(form: web::Form<FormData>) -> String {
    format!("Welcome {}!, Email {}", form.email, form.name)
}

#[post("/subscriptions")]
pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> impl Responder {
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4) 
        "#,
        Uuid::new_v4(), 
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pool.get_ref())
    .await 
    {   
        Ok(_) => HttpResponse::Ok().finish(), 
        Err(e) => {
            println!("Failed to execute query: {}", e);
            HttpResponse::InternalServerError().finish() 
        }
    }
}
