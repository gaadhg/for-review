use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use smol_str::{SmolStr, ToSmolStr};
use thiserror::Error;

use crate::domain::*;
#[derive(Debug, Error)]
pub enum EventError {
    #[error("name is longer than 64 characters")]
    NameTooLong,
    #[error("description is longer than 255 characters")]
    DescriptionTooLong,
    #[error("did the clock start to go backwards or?")]
    InvalidTimestamp,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Event {
    id: UUIDv7,

    name: SmolStr,
    description: SmolStr,

    label: Option<Subject>,

    timestamp: DateTime<Utc>,
}

impl Event {
    pub(in crate::domain) fn new(
        id: UUIDv7,
        name: &str,
        description: &str,
        label: Option<Subject>,
        timestamp: DateTime<Utc>,
    ) -> Result<Self, EventError> {
        if name.len() > 64 {
            return Err(EventError::NameTooLong);
        }
        if description.len() > 255 {
            return Err(EventError::DescriptionTooLong);
        }

        if timestamp.date_naive() <= Utc::now().date_naive() {
            return Err(EventError::InvalidTimestamp);
        }

        Ok(Self {
            id,
            name: name.to_smolstr(),
            description: description.to_smolstr(),
            label,
            timestamp,
        })
    }

    pub fn id(&self) -> UUIDv7 {
        self.id.clone()
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn label(&self) -> Option<&Subject> {
        self.label.as_ref()
    }

    pub fn timestamp(&self) -> &DateTime<Utc> {
        &self.timestamp
    }
}
