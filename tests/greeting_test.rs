//! tests/greeting.rs
mod common;

mod tests {
    use crate::common::spawn_app;

    #[tokio::test]
    async fn greeting_succeed() {
        // Arrange
        let app = spawn_app().await;
        let client = reqwest::Client::new();

        // Act
        let server = client
            .get(&format!("{}/greeting", &app.address))
            .send()
            .await
            .expect("Failed to execute request.");

        // Asserts
        assert!(server.status().is_success())
    }
}
