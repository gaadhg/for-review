use std::time::SystemTime;

use native_db::{Key, ToKey};
use rand::Rng;
use serde::{Deserialize, Serialize};

pub enum UUIDError {
    TimeError,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub struct UUIDv7(u128);

impl ToKey for UUIDv7 {
    fn to_key(&self) -> native_db::Key {
        Key::new(self.0.to_le_bytes().to_vec())
    }

    fn key_names() -> Vec<String> {
        vec!["UUID".to_string()]
    }
}

impl UUIDv7 {
    const TIME_BITS: u8 = 48;
    const RAND_BITS: u8 = 80;

    pub fn new() -> Result<Self, UUIDError> {
        UUIDv7::from_datetime(SystemTime::now())
    }

    pub fn from_datetime(datetime: SystemTime) -> Result<Self, UUIDError> {
        let mut rng = rand::thread_rng();

        let timestamp = datetime
            .duration_since(SystemTime::UNIX_EPOCH)
            .map_err(|_| UUIDError::TimeError)?
            .as_millis();
        let timestamp_48bit = (timestamp & 0xFFFFFFFFFFFF) as u128;

        let upper: u64 = rng.r#gen();
        let lower: u16 = rng.r#gen();
        let random_80bit: u128 = ((upper as u128) << 16) | (lower as u128);

        Ok(UUIDv7((timestamp_48bit << Self::RAND_BITS) | random_80bit))
    }

    pub fn as_u128(&self) -> u128 {
        self.0
    }
}

// impl From<UUIDv7> for u128 {
//     fn from(value: UUIDv7) -> Self {
//         value.get()
//     }
// }

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
