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

// curl i- -X POST -d 'email=rush5doshi%40gmail.com&name=Doshi' http://127.0.0.1:8000/subscriptions
#[post("/subscriptions")]
pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> impl Responder {
    let request_id = Uuid::new_v4();

    tracing::info!(
        "request_id={} - Adding a new subscriber. name={}, email={}",
            request_id, 
            form.name,
            form.email,
    );

    tracing::info!(
        "request_id={} - Saving new subscriber details in the database",
        request_id
    );

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
        Ok(_) => {
            tracing::info!(
                "request_id={} - New subscriber details have been saved.",
                request_id,
            );
            HttpResponse::Ok().finish()
        }, 
        Err(e) => {
            tracing::error!(
                "request_id={} - Failed to execute query: {:?}",
                request_id,
                e,
            );
            HttpResponse::InternalServerError().finish() 
        }
    }
}
