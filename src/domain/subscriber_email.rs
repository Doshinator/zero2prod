//! src/domain/subscriber_email.rs

use validator::ValidateEmail;

#[derive(Debug)]
pub struct SubscriberEmail(String);
impl SubscriberEmail {
    pub fn parse(s: String) -> Result<SubscriberEmail, String> {
        if s.validate_email() {
            Ok(Self(s))
        } 
        else {
            Err(format!("{} is not a valid subscriber email.", s))
        }
    } 
}

impl AsRef<str> for SubscriberEmail {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::SubscriberEmail;
    use claims::assert_err;
    
    #[test]
    fn empty_string_is_rejeted() {
        // Arrange
        let email = "".to_string();
        
        // Act
        let result = SubscriberEmail::parse(email);
        
        // Assert
        assert_err!(result);
    }

    #[test]
    fn email_missing_at_symbol_is_rejected() {
        // Arrange
        let email = "invalidemail.com".to_string();

        // Act
        let result = SubscriberEmail::parse(email);
        
        // Assert
        assert_err!(result);
    }

    #[test]
    fn email_missing_subjet_is_rejected() {
        // Arrange
        let email = "@gmail.com".to_string();

        // Act
        let result = SubscriberEmail::parse(email);
        
        // Assert
        assert_err!(result);
    }
}
