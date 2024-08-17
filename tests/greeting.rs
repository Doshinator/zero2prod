mod common;

mod tests {
    use crate::common;

    #[tokio::test]
    async fn greeting_succeed() {
        // Arrange
        let addrs = common::spawn_app();
        let client = reqwest::Client::new();

        // Act
        let server = client
            .get(&format!("{}/greeting", &addrs))
            .send()
            .await
            .expect("Failed to execute request.");

        // Asserts
        assert!(server.status().is_success())
    }
}
