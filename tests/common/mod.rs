// - if we try to run two or more tests in parallel only one of them will manage to bind the port,
// all others will fail.

// - if port 8000 is being used by another program on our machine (e.g. our own application!), tests will fail;
// so this test will fail!!

use zero2prod::configuration::get_configuration;
use std::net::TcpListener;
use sqlx::PgPool;

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

    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to bind address");

    let server = zero2prod::startup::run(listener, connection_pool.clone())
        .expect("Failed to bind address");
    let _ = tokio::spawn(server);

    TestApp {
        address,
        db_pool: connection_pool,
    }
}
