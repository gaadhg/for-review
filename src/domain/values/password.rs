use arrayvec::ArrayString;
use bcrypt::*;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PasswordError {
    #[error("password longer than 72 characters")]
    TooLong,
    #[error("password incorrect")]
    IncorrectPassword,
    #[error("internal error")]
    Internal,
}

impl From<BcryptError> for PasswordError {
    fn from(value: BcryptError) -> Self {
        match value {
            BcryptError::Truncation(_) => PasswordError::TooLong,
            _ => PasswordError::Internal,
        }
    }
}

const MAX_HASH_LENGTH: usize = 60;
const COST: u32 = 10;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Password(ArrayString<MAX_HASH_LENGTH>);

impl Password {
    pub(in crate::domain) fn new<P: AsRef<[u8]>>(password: P) -> Result<Password, PasswordError> {
        let hash = non_truncating_hash(password, COST)?;
        Ok(Password(
            ArrayString::from(&hash).map_err(|_| PasswordError::Internal)?,
        ))
    }

    pub(in crate::domain) fn validate<P: AsRef<[u8]>>(&self, input: P) -> bool {
        if let Ok(is_valid) = verify(input, &self.0) {
            return is_valid;
        }
        false
    }

    // pub(in crate::domain) fn change<P: AsRef<[u8]>>(&mut self, new: P) -> Result<(), PasswordError> {
    //     let hash = non_truncating_hash(new, COST)?;
    //     self.0 = ArrayString::from(&hash).map_err(|_| PasswordError::Internal)?;
    //     Ok(())
    // }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

#[cfg(test)]
mod test {
    use crate::domain::values::password::MAX_HASH_LENGTH;

    use super::Password;

    #[test]
    pub fn create_password() {
        let hash = Password::new("test random words").unwrap();
        assert_eq!(hash.0.len(), MAX_HASH_LENGTH);
        assert!(hash.validate("test random words"))
    }

    #[test]
    #[should_panic]
    pub fn panics_too_long() {
        let password = Password::new("a".repeat(80)).unwrap();
    }
}
