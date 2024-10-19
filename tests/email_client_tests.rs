//! src/tests/email_client_tests.rs

#[cfg(test)]
mod tests {
    use claims::{assert_ok, assert_err};
    use fake::faker::internet::en::SafeEmail;
    use fake::faker::lorem::en::{Paragraph, Sentence};
    use fake::{Fake, Faker};
    use secrecy::Secret;
    use wiremock::matchers::{any, header, header_exists, method, path};
    use wiremock::{Mock, MockServer, Request, ResponseTemplate};
    
    use zero2prod::{domain::SubscriberEmail, email_client::EmailClient};

    // Custom matcher to use in Mock to check if body is valid json & contains set of field names
    struct SendEmailBodyMatch;

    impl wiremock::Match for SendEmailBodyMatch {
        fn matches(&self, request: &Request) -> bool {
            let result: Result<serde_json::Value, _> = serde_json::from_slice(&request.body);
            if let Ok(body) = result {
                dbg!(&body);
                // Check that all the mandatory fields are populated
                // without inspecting the field values
                body.get("From").is_some()
                    && body.get("To").is_some()
                    && body.get("Subject").is_some()
                    && body.get("HtmlBody").is_some()
                    && body.get("Text").is_some()
            }
            else {
                false
            }
        }
    }

    #[tokio::test]
    async fn send_email_sends_the_expected_request() {
        // Arrange
        let mock_server = MockServer::start().await;
        // pass mock server uri to our client
        let email_client = email_client(mock_server.uri());

        Mock::given(header_exists("X-Postmark-Server-Token"))
            .and(header("Content-Type", "application/json"))
            .and(path("/messages"))
            .and(method("POST"))
            .and(SendEmailBodyMatch)
            .respond_with(ResponseTemplate::new(200))
            .expect(1)
            .mount(&mock_server)
            .await;

        // Act
        let result = email_client
            .send_email(email(), &subject(), &content(), &content())
            .await;

        // Assert
        assert_ok!(result);
    }

    #[tokio::test]
    async fn send_email_succeeds_if_the_server_returns_200() {
        // Arrange
        let mock_server = MockServer::start().await;
        let email_client = email_client(mock_server.uri());
    
        Mock::given(any())
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&mock_server)
        .await;

        // Act
        let result = email_client
            .send_email(email(), &subject(), &content(), &content())
            .await;

        // Asserts
        assert_ok!(result);
    }

    #[tokio::test]
    async fn send_email_fails_if_the_server_returns_500() {
        // Arrange
        let mock_server = MockServer::start().await;
        let email_client = email_client(mock_server.uri());

        let subscriber_email = email();
        let subject = subject();
        let content = content();

        Mock::given(any())
            .respond_with(ResponseTemplate::new(500))
            .expect(1)
            .mount(&mock_server)
            .await;

        // Act
        let result = email_client
            .send_email(subscriber_email, &subject, &content, &content)
            .await;

        // Asserts
        assert_err!(result);
    }

    // Helper functions
    fn subject() -> String {
        Sentence(1..2).fake()
    }

    fn content() -> String {
        Paragraph(1..10).fake()
    }

    fn email() -> SubscriberEmail {
        SubscriberEmail::parse(SafeEmail().fake()).unwrap()
    }

    fn email_client(base_uri: String) -> EmailClient {
        EmailClient::new(
            base_uri,
            email(),
            Secret::new(Faker.fake()),
            std::time::Duration::from_millis(200)
        )
    }
}
