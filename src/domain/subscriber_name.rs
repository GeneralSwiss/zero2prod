use unicode_segmentation::UnicodeSegmentation;

#[derive(serde::Deserialize, Debug)]
#[serde(try_from = "String")]
pub struct SubscriberName(String);

impl TryFrom<String> for SubscriberName {
    type Error = String;
    fn try_from(input: String) -> Result<Self, Self::Error> {
        let is_empty_or_whitespace = input.trim().is_empty();
        let is_too_long = input.graphemes(true).count() > 256;
        let forbidden_chars = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
        let contains_forbidden_chars = input.chars().any(|it| forbidden_chars.contains(&it));
        if is_empty_or_whitespace || is_too_long || contains_forbidden_chars {
            Err(format!("{} is not a valid subscriber name.", input))
        } else {
            Ok(Self(input))
        }
    }
}

impl AsRef<str> for SubscriberName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::SubscriberName;
    use claim::{assert_err, assert_ok};

    #[test]
    fn a_256_grapheme_long_name_is_valid() {
        let name = "ë".repeat(256);
        assert_ok!(SubscriberName::try_from(name));
    }

    #[test]
    fn a_name_longer_than_256_graphemes_is_rejected() {
        let name = "ë".repeat(300);
        assert_err!(SubscriberName::try_from(name));
    }

    #[test]
    fn whitespace_only_names_are_rejected() {
        let name = " ".repeat(3);
        assert_err!(SubscriberName::try_from(name));
    }

    #[test]
    fn empty_string_is_rejected() {
        let empty_name = "".to_string();
        assert_err!(SubscriberName::try_from(empty_name));
    }

    #[test]
    fn names_with_invalid_characters_are_rejected() {
        for name in &['/', '(', ')', '"', '<', '>', '\\', '{', '}'] {
            let name = name.to_string();
            assert_err!(SubscriberName::try_from(name));
        }
    }
    #[test]
    fn names_with_valid_characters_are_accepted() {
        let name = "Tim".to_string();
        assert_ok!(SubscriberName::try_from(name));
    }
}
