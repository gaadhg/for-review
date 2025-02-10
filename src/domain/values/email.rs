use native_db::{Key, ToKey};
use regex_static::{lazy_regex, once_cell::sync::Lazy};
use smol_str::{SmolStr, ToSmolStr};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum EmailError {
    #[error("malformated email")]
    InvalidEmail,
    #[error("email too long")]
    TooLong,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Email(SmolStr);

// Pazi boje
static VALIDATOR: Lazy<regex::Regex> = regex_static::lazy_regex!(
    r#"(?:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\])"#
);

impl Email {
    pub fn new(input: &str) -> Result<Self, EmailError> {
        if VALIDATOR.is_match(input) {
            return Ok(Email(input.to_smolstr()));
        }

        Err(EmailError::InvalidEmail)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl ToKey for Email {
    fn to_key(&self) -> native_db::Key {
        Key::new(self.0.as_bytes().to_vec())
    }

    fn key_names() -> Vec<String> {
        vec!["Email".to_string()]
    }
}
