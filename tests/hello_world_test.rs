//! tests/hello_world.rs
mod common;

mod tests {
    use crate::common;

    #[tokio::test]
    async fn hello_world_success() {
        // Arrange
        let addrs = common::spawn_app();
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
