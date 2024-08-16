//! tests/health_check.rs
//! no need to add $[cfg(test)],
//! Cargo treats the tests directory specially and compiles files in this directory only when we run cargo test

mod tests {
    use crate::spawn_app;

    #[rustfmt::skip]
    #[tokio::test] 
    async fn health_check_success() {
        // Arrange
        spawn_app();

        // Act
        let client = reqwest::Client::new();
        let response = client
            .get("http://0.0.0.0:8080/health_check")
            .send()
            .await
            .expect("Failed to execute request.");
    
        // Asserts
        assert!(response.status().is_success());
        assert_eq!(Some(0), response.content_length());
    }
}

fn spawn_app() {
    let server = zero2prod::run().expect("Failed to bind address");
    let _ = tokio::spawn(server);
}
