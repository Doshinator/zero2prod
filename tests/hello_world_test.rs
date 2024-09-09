//! tests/hello_world.rs
mod common;

mod tests {
    use crate::common::spawn_app;

    #[tokio::test]
    async fn hello_world_success() {
        // Arrange
        let app = spawn_app().await;
        let client = reqwest::Client::new();

        // Act
        let response = client
            .get(&format!("{}/ ", &app.address))
            .send()
            .await
            .expect("Failed to execute request.");

        // Asserts
        assert!(response.status().is_success());
    }
}
