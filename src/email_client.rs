//! src/email_client.rs

use crate::domain::SubscriberEmail;
use actix_web::rt::time;
use config::builder;
use reqwest::{Client, ClientBuilder};
use secrecy::{ExposeSecret, Secret};
use serde::{Deserialize, Serialize};
use tokio::time::timeout;

// email client model
pub struct EmailClient {
    http_client: Client,
    base_url: String,
    sender: SubscriberEmail,
    auth_token: Secret<String>,
}

// controller implementations
impl EmailClient {
    pub fn new(
        base_url: String,
        sender: SubscriberEmail,
        auth_token: Secret<String>,
        timeout: std::time::Duration
    ) -> Self {
        let http_client = Client::builder().timeout(timeout).build().unwrap();
        Self {
            http_client,
            base_url,
            sender,
            auth_token,
        }
    }

    pub async fn send_email(
        &self,
        recepient: SubscriberEmail,
        subject: &str,
        html_content: &str,
        text_content: &str,
    ) -> Result<(), reqwest::Error> {
        let url = format!("{}/messages", self.base_url);
        let request_body = SendEmailRequestBody {
            from: self.sender.as_ref(),
            to: recepient.as_ref(),
            subject: subject,
            html_body: html_content,
            text: text_content,
        };

        let builder = self
            .http_client
            .post(&url)
            .header(
                "X-Postmark-Server-Token",
                self.auth_token.expose_secret()
            )
            .json(&request_body)
            .send() // send returns Ok as long as it gets a valid response from the serve, no matter the status code
            .await?
            .error_for_status()?;
        Ok(())
    }
}

// json request body for email request
#[derive(serde::Serialize)]
#[serde(rename_all = "PascalCase")]
struct SendEmailRequestBody<'a> {
    from: &'a str,
    to: &'a str,
    subject: &'a str,
    html_body: &'a str,
    text: &'a str,
}
