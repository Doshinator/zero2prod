//! src/routes/subscriptions.rs
use actix_web::{post, web, HttpResponse, Responder};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::{NewSubscriber, SubscriberEmail, SubscriberName};

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

#[allow(dead_code)]
fn index(form: web::Form<FormData>) -> String {
    format!("Welcome {}!, Email {}", form.email, form.name)
}

#[allow(clippy::async_yields_async)]
#[tracing::instrument(
    // name can be used to specify the message associated to the function span - if omitted, it defaults to the function name.
    name = "Adding a new subscriber", 
    skip(form, pool),
    fields(
        subscriber_email = %form.email,
        subscriber_name= %form.name
    )
)]
// curl -i -X POST -d 'email=rush5doshi%40gmail.com&name=Doshi' http://127.0.0.1:8000/subscriptions
#[post("/subscriptions")]
pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> impl Responder {
    let email = match SubscriberEmail::parse(form.0.email) {
        Ok(email) => email,
        // Return early if the name is invalid, with a 400
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    let name = match SubscriberName::parse(form.0.name) {
        Ok(name) => name,
        // Return early if the name is invalid, with a 400
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    let new_subscriber = NewSubscriber {
        email,
        name,
    };

    match insert_subscriber(&new_subscriber, &pool).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(new_subscriber, pool)
)]
pub async fn insert_subscriber(
    new_subscriber: &NewSubscriber,
    pool: &PgPool,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4) 
        "#,
        Uuid::new_v4(),
        new_subscriber.email.as_ref(),
        new_subscriber.name.as_ref(),
        Utc::now()
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    Ok(())
}
