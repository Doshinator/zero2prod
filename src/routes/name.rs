//! src/routes/name.rs
use actix_web::{post, web, HttpResponse, Responder};
use sqlx::PgPool;
use chrono::Utc; 
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

// // to test post enpoint, in terminal
// // curl -X POST 127.0.0.1:8000/name -H "Content-Type: plain/text" -d "request body here"
#[post("/name")]
pub async fn name(form: web::Form<FormData>, pool: web::Data<PgPool>) -> impl Responder {
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
