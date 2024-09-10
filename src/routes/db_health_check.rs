//! src/routes/health_check.rs
use actix_web::{error::ErrorInternalServerError, get, web::Data, HttpResponse, Responder};
use sqlx::PgPool;

#[get("/db_health_check")]
pub async fn db_health_check(pool: Data<PgPool>) -> impl Responder {
    match sqlx::query!(
        r#"
        SELECT 1 AS result
        "#,
    )
    .fetch_one(pool.get_ref())
    .await
    {
        Ok(record) => {
            let result = record.result;
            HttpResponse::Ok().body(format!("Database is healthy! {:?}", result))
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
