mod common;

mod tests {
    use crate::common::spawn_app;

    #[tokio::test]
    async fn name_success() {
        // Arrange
        let app = spawn_app().await;
        let client = reqwest::Client::new();
        let body = "name=Doshi&email=rush5doshi%40gmail.com";

        // Act
        let response = client
            .post(&format!("{}/name", &app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            
        // Asserts
    }
}