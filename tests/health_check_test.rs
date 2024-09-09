//! tests/health_check.rs
//! no need to add $[cfg(test)],
//! Cargo treats the tests directory specially and compiles files in this directory only when we run cargo test

mod common;

mod tests {
    use crate::common::spawn_app;

    #[rustfmt::skip]
    #[tokio::test] 
    async fn health_check_success() {
        // Arrange
        let app = spawn_app().await;
        let client = reqwest::Client::new();

        // Act
        let response = client
            .get(&format!("{}/health_check", &app.address))
            .send()
            .await
            .expect("Failed to execute request.");
    
        // Asserts
        assert!(response.status().is_success());
        assert_eq!(Some(0), response.content_length());
    }
}

mod subscriber_test {
    use crate::common::spawn_app;

    #[tokio::test]
    async fn subscriber_returns_200_ok() {
        // Arrange
        let app = spawn_app().await;
        let client = reqwest::Client::new();
        let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";

        // Act
        let response = client
            .post(&format!("{}/subscriptions", &app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("Failed to execute request.");

        // Assert
        assert_eq!(200, response.status().as_u16());
        let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
            .fetch_one(&app.db_pool)
            .await
            .expect("Failed to fetch saved subscription.");

        assert_eq!("ursula_le_guin@gmail.com", saved.email);
        assert_eq!("le guin", saved.name);
    }

    #[tokio::test]
    async fn subscriber_returns_400_data_missing() {
        // Arrange
        let app = spawn_app().await;
        let client = reqwest::Client::new();
        let test_cases: Vec<(&str, &str)> = vec![
            ("name=le%20guin", "missing the email"),
            ("email=ursula_le_guin%40gmail.com", "missing the name"),
            ("", "missing both name and email"),
        ];

        for (invalid_body, error_message) in test_cases {
            // Act
            let response = client
                .post(&format!("{}/subscriptions", &app.address))
                .header("Content-Type", "application/x-www-form-urlencoded")
                .body(invalid_body)
                .send()
                .await
                .expect("Failed to execute request.");

            // Assserts
            assert_eq!(
                400,
                response.status().as_u16(),
                "The API did not fail with 400 Bad Request when the payload was {}.",
                error_message
            );
        }
    }
}
