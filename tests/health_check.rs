//! tests/health_check.rs
//! no need to add $[cfg(test)],
//! Cargo treats the tests directory specially and compiles files in this directory only when we run cargo test

use std::net::TcpListener;

mod tests {
    use crate::spawn_app;

    #[rustfmt::skip]
    #[tokio::test] 
    async fn health_check_success() {
        // Arrange
        let addrs = spawn_app();
        let client = reqwest::Client::new();

        // Act
        let response = client
            .get(&format!("{}/health_check", &addrs))
            .send()
            .await
            .expect("Failed to execute request.");
    
        // Asserts
        assert!(response.status().is_success());
        assert_eq!(Some(0), response.content_length());
    }
    
    #[tokio::test]
    async fn hello_world_success() {
        // Arrange
        let addrs = spawn_app();
        let client = reqwest::Client::new();

        // Act
        let response = client
            .get(&format!("{}/ ", &addrs))
            .send()
            .await
            .expect("Failed to execute request.");

        // Asserts
        assert!(response.status().is_success());
    }
}

// - if we try to run two or more tests in parallel only one of them will manage to bind the port, 
// all others will fail.

// - if port 8000 is being used by another program on our machine (e.g. our own application!), tests will fail;
// so this test will fail!!    
fn spawn_app() -> String {
    let addrs: &str = "127.0.0.1:0";
    let listener = TcpListener::bind(addrs)
        .expect("failed to bind address");
    let port = listener.local_addr().unwrap().port();

    let server = zero2prod::run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}
