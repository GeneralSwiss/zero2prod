#[derive(serde::Deserialize, Debug)]
#[serde(try_from = "String")]
pub struct SubscriberEmail(String);

impl AsRef<str> for SubscriberEmail {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for SubscriberEmail {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if validator::validate_email(&value) {
            Ok(SubscriberEmail(value))
        } else {
            Err(format!("{} was an invalid email address.", value))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SubscriberEmail;
    use claim::assert_err;
    use fake::faker::internet::en::SafeEmail;
    use fake::Fake;

    #[test]
    fn empty_string_is_rejected() {
        assert_err!(SubscriberEmail::try_from("".to_string()));
    }

    #[test]
    fn email_missing_at_symbol_is_rejected() {
        let missing_at_symbol = "jim-google.com".to_string();
        assert_err!(SubscriberEmail::try_from(missing_at_symbol));
    }

    #[test]
    fn email_missing_subject_is_rejected() {
        let missing_subject = "@yahoo.com".to_string();
        assert_err!(SubscriberEmail::try_from(missing_subject));
    }

    #[derive(Debug, Clone)]
    struct ValidEmailFixture(pub String);

    impl quickcheck::Arbitrary for ValidEmailFixture {
        fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> Self {
            Self(SafeEmail().fake_with_rng(g))
        }
    }

    #[quickcheck_macros::quickcheck]
    fn valid_emails_are_parsed_successfully(valid_email: ValidEmailFixture) -> bool {
        SubscriberEmail::try_from(valid_email.0).is_ok()
    }
}
