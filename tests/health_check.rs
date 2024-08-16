//! tests/health_check.rs
//! no need to add $[cfg(test)],
//! Cargo treats the tests directory specially and compiles files in this directory only when we run cargo test

mod common;

mod tests {
    use crate::common;

    #[rustfmt::skip]
    #[tokio::test] 
    async fn health_check_success() {
        // Arrange
        let addrs = common::spawn_app();
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
}
