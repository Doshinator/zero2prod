//! src/domain/subscriber_email.rs

use validator::ValidateEmail;

#[derive(Debug)]
pub struct SubscriberEmail(String);

impl SubscriberEmail {
    pub fn parse(s: String) -> Result<SubscriberEmail, String> {
        if s.validate_email() {
            Ok(Self(s))
        } else {
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
    use fake::faker::internet::en::SafeEmail;
    use fake::Fake;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

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

    #[derive(Debug, Clone)]
    struct ValidEmailFixture(pub String);

    impl quickcheck::Arbitrary for ValidEmailFixture {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            let mut rng = StdRng::seed_from_u64(u64::arbitrary(g));
            let email = SafeEmail().fake_with_rng(&mut rng);

            Self(email)
        }
    }

    #[quickcheck_macros::quickcheck]
    fn valid_emails_are_parsed_successfully(valid_email: ValidEmailFixture) -> bool {
        // Act & Asserts
        // to see email outputs for test
        // cargo test valid_emails -- --nocapture
        // dbg!(&valid_email.0);
        SubscriberEmail::parse(valid_email.0).is_ok()
    }
}
