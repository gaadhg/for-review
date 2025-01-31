use std::time::SystemTime;

use native_db::{Key, ToKey};
use rand::Rng;
use serde::{Deserialize, Serialize};

pub enum UUIDError {
    TimeError,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct UUID(u128);

impl ToKey for UUID {
    fn to_key(&self) -> native_db::Key {
        Key::new(self.0.to_le_bytes().to_vec())
    }

    fn key_names() -> Vec<String> {
        vec!["UUID".to_string()]
    }
}

impl UUID {
    const TIME_BITS: u8 = 48;
    const RAND_BITS: u8 = 80;

    pub fn new() -> Result<Self, UUIDError> {
        UUID::from_datetime(SystemTime::now())
    }

    pub fn from_datetime(datetime: SystemTime) -> Result<Self, UUIDError> {
        let mut rng = rand::thread_rng();

        let timestamp = 
            datetime
                .duration_since(SystemTime::UNIX_EPOCH)
                .map_err(|_| { UUIDError::TimeError })?
                .as_millis();
        let timestamp_48bit = (timestamp & 0xFFFFFFFFFFFF) as u128;

        let upper: u64 = rng.r#gen();
        let lower: u16 = rng.r#gen();
        let random_80bit: u128 = ((upper as u128) << 16) | (lower as u128);

        Ok(UUID((timestamp_48bit << Self::RAND_BITS) | random_80bit))
    }

    pub fn get(&self) -> u128 {
        self.0
    }
}

impl Into<u128> for UUID {
    fn into(self) -> u128 {
        self.get()
    }
}


mod test {
    use super::UUID;

    fn print_uuid() {
        let Ok(uuid) = UUID::new() else {panic!("Error generating UUID")} ;
        
        println!("{}", uuid.get());
    }
}