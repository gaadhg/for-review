use std::{fmt::Display, time::SystemTime};

use native_db::{Key, ToKey};
use rand::Rng;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum UUIDError {
    #[error("Internal Error")]
    InternalError,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
// TODO: Investigate why when instead of string I store u128 (AS I SHOULD) axum-login crate goes nutz.
// TODO: Maybe use ArrayStr
pub struct UUIDv7(String);

impl UUIDv7 {
    const TIME_BITS: u8 = 48;
    const RAND_BITS: u8 = 80;

    pub(in crate::domain) fn new() -> Result<Self, UUIDError> {
        UUIDv7::from_datetime(SystemTime::now())
    }

    pub(in crate::domain) fn from_datetime(datetime: SystemTime) -> Result<Self, UUIDError> {
        let mut rng = rand::thread_rng();

        let timestamp = datetime
            .duration_since(SystemTime::UNIX_EPOCH)
            .map_err(|_| UUIDError::InternalError)?
            .as_millis();

        #[allow(clippy::unnecessary_cast)]
        let timestamp_48bit = (timestamp & 0xFFFFFFFFFFFF) as u128;

        let upper: u64 = rng.r#gen();
        let lower: u16 = rng.r#gen();
        let random_80bit: u128 = ((upper as u128) << 16) | (lower as u128);

        Ok(UUIDv7(
            ((timestamp_48bit << Self::RAND_BITS) | random_80bit).to_string(),
        ))
    }

    pub fn from_u128(uuid: u128) -> UUIDv7 {
        Self(uuid.to_string())
    }

    pub fn as_u128(&self) -> u128 {
        self.0.parse().unwrap()
    }
}

impl ToKey for UUIDv7 {
    fn to_key(&self) -> native_db::Key {
        Key::new(self.0.parse::<u128>().unwrap().to_le_bytes().to_vec())
    }

    fn key_names() -> Vec<String> {
        vec!["UUID".to_string()]
    }
}

impl Display for UUIDv7 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod test {
    use super::UUIDv7;

    #[test]
    fn print_uuid() {
        let Ok(uuid) = UUIDv7::new() else {
            panic!("Error generating UUID")
        };

        println!("{}", uuid.as_u128());
    }
}
