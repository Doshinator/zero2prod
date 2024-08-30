//! src/routes/name.rs
use std::path;

use actix_web::{get, post, web, HttpResponse, Responder};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct Subscription {
    email: String,
    name: String,
}

// curl -X GET http://127.0.0.1:8000/name/le%20guin
// Query result: Record { id: 51dde2ad-a02e-49c6-8c87-9b472a3621da, email: "ursula_le_guin@gmail.com", name: "le guin" }
#[get("/name/{name}")]
pub async fn name(
    path: web::Path<String>, // Extract the `name` parameter from the URL
    pool: web::Data<PgPool>, // Database connection pool
) -> impl Responder {
    let name = path.into_inner(); // Extract the `name` from the path

    // Perform the SQL query
    match sqlx::query!(
        r#"
        SELECT id, email, name
        FROM subscriptions 
        WHERE name = $1
        "#,
        name
    )
    .fetch_one(pool.get_ref()) // Fetch a single record
    .await
    {
        Ok(record) => {
            let request_span = tracing::info_span!(
                "Fetching Record",
                subscriber_name = %record.name,
                subscriber_email = %record.email,
            );
            let _request_span_guard = request_span.enter();

            HttpResponse::Ok().finish()
        }
        Err(e) => {
            println!("Failed to execute query: {}", e);
            HttpResponse::InternalServerError().finish() // Return an error if the query fails
        }
    }
}
