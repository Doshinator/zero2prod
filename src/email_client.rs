//! src/email_client.rs

use crate::domain::SubscriberEmail;
use config::builder;
use reqwest::Client;
use secrecy::Secret;
use serde::{Deserialize, Serialize};

// email client model
pub struct EmailClient {
    http_client: Client,
    base_url: String,
    sender: SubscriberEmail,
    // auth_token: Secret<String>
}

// controller implementations
impl EmailClient {
    pub fn new(
        base_url: String,
        sender: SubscriberEmail,
        // auth_token: Secret<String>,
    ) -> Self {
        Self {
            http_client: Client::new(),
            base_url,
            sender,
            // auth_token
        }
    }

    pub async fn send_email(
        &self,
        recepient: SubscriberEmail,
        subject: &str,
        html_content: &str,
        text_content: &str,
    ) -> Result<(), String> {
        let url = format!("{}/messages", self.base_url);
        let request_body = SendEmailRequestBody {
            from: self.sender.as_ref().to_owned(),
            to: recepient.as_ref().to_owned(),
            subject: subject.to_owned(),
            html_body: html_content.to_owned(),
            text: text_content.to_owned(),
        };

        let builder = self
            .http_client
            .post(&url)
            .json(&request_body);

        Ok(())
    }
}

// json request body for email request
#[derive(serde::Serialize)]
struct SendEmailRequestBody {
    from: String,
    to: String,
    subject: String,
    html_body: String,
    text: String,
}
