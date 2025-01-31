use bcrypt::{non_truncating_hash, non_truncating_hash_with_result, non_truncating_verify, verify, BcryptError};

#[derive(Debug)]
pub enum PasswordError {
    TooLong,
    IncorrectPassword,
    InternalError
}

impl From<BcryptError> for PasswordError {
    fn from(value: BcryptError) -> Self {
        match value {
            BcryptError::Truncation(_) => PasswordError::TooLong,
            _ => PasswordError::InternalError
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Password(String);

impl Password {
    pub fn new<P: AsRef<[u8]>>(password: P) -> Result<Password, PasswordError> {
        // if password.as_ref().len() > 72 { return Err(PasswordError::TooLong); }
        Ok(Password(non_truncating_hash(password, 10)?))
    }

    pub fn validate<P: AsRef<[u8]>>(&self, input: P) -> Result<bool, PasswordError> {
        // if input.as_ref().len() > 72 { return Err(PasswordError::TooLong); }
        Ok(non_truncating_verify(input, &self.0)?)
    }

    pub fn change<P: AsRef<[u8]>>(&mut self, new: P) -> Result<(), PasswordError> {
        Ok(self.0 = non_truncating_hash(new, 10)?)
    }
}