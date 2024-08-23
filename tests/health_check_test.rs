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

mod subscriber_test {
    use crate::common::spawn_app;
    use sqlx::{Connection, PgConnection};
    use zero2prod::configuration::get_configuration;

    #[tokio::test]
    async fn subscriber_returns_200_ok() {
        // Arrange
        let addrs = spawn_app();
        let configuration = get_configuration().expect("Failed to read configuration.");
        let configuration_string = configuration.database.connection_string();
        let mut connection = PgConnection::connect(&configuration_string)
            .await
            .expect("Failed to connect to Postgres.");
        let client = reqwest::Client::new();
        let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";

        // Act
        let response = client
            .post(&format!("{}/subscriptions", &addrs))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("Failed to execute request.");

        // Assert
        assert_eq!(200, response.status().as_u16());
        let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
            .fetch_one(&mut connection)
            .await
            .expect("Failed to fetch saved subscription.");
        
        assert_eq!("ursula_le_guin@gmail.com", saved.email);
        assert_eq!("le guin", saved.name);
    }

    #[tokio::test]
    async fn subscriber_returns_400_data_missing() {
        // Arrange
        let addrs = spawn_app();
        let client = reqwest::Client::new();
        let test_cases: Vec<(&str, &str)> = vec![
            ("name=le%20guin", "missing the email"),
            ("email=ursula_le_guin%40gmail.com", "missing the name"),
            ("", "missing both name and email"),
        ];

        for (invalid_body, error_message) in test_cases {
            // Act
            let response = client
                .post(&format!("{}/subscriptions", &addrs))
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
