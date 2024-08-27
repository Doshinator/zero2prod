// - if we try to run two or more tests in parallel only one of them will manage to bind the port,
// all others will fail.

// - if port 8000 is being used by another program on our machine (e.g. our own application!), tests will fail;
// so this test will fail!!

use zero2prod::configuration::{get_configuration, DatabaseSettings};
use std::net::TcpListener;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

pub async fn spawn_app() -> TestApp {
    // port 0 to be able to randomize port assignemnt
    let addrs: &str = "127.0.0.1:0";
    let listener = TcpListener::bind(addrs).expect("failed to bind address");
        let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    let mut configuration = get_configuration().expect("Failed to read configuration.");
    configuration.database.database_name = Uuid::new_v4().to_string();

    let connection_pool = configure_database(&configuration.database)
        .await;

    let server = zero2prod::startup::run(listener, connection_pool.clone())
        .expect("Failed to bind address");
    let _ = tokio::spawn(server);

    TestApp {
        address,
        db_pool: connection_pool,
    }
}

pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    // Create database
    let mut connection = PgConnection::connect(&config.connection_string_without_db())
        .await
        .expect("Failed to connect to Postgres");
    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name)
        .as_str())
        .await
        .expect("Failed to create database.");

    // Migrate database
    let connection_pool = PgPool::connect(&config.connection_string())
        .await
        .expect("Failed to connect to Postgres."); 
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");

    connection_pool
}
