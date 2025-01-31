use core::time;

use chrono::{DateTime, Utc};

use super::*;

#[derive(Debug)]
pub enum EventError {
    NameTooLong,
    DescriptionTooLong,
    EventNotFound,
    InternalError,
}


#[derive(serde::Serialize, serde::Deserialize)]
pub struct Event {
    id: UUID,

    name: String,
    description: String,
    
    label: Option<Subject>,

    timestamp: DateTime<Utc>
}

impl Event {
    pub fn new(name: String, description: String, time: DateTime<Utc>, label: Option<Subject>) -> Result<Self, EventError> {
        if name.len() > 64 { return Err(EventError::NameTooLong) }
        if description.len() > 256 { return Err(EventError::DescriptionTooLong) }
        let Ok(uuid) = UUID::new() else { return Err(EventError::InternalError) };
        
        Ok(Self {
            id: uuid,
            name: name,
            description: description,
            timestamp: time,
            label: label
        })
    }

    pub fn update_event(&mut self, name: Option<String>, description: Option<String>, label: Option<Option<Subject>>, timestamp: Option<DateTime<Utc>>) -> Result<(), EventError> {
        if let Some(name) = name {
            if name.len() > 64 { return Err(EventError::NameTooLong) }
            self.name = name;
        }

        if let Some(description) = description {
            if description.len() > 256 { return Err(EventError::DescriptionTooLong) }
            self.description = description;
        }

        if let Some(label) = label {
            self.label = label;
        }

        if let Some(timestamp) = timestamp {
            self.timestamp = timestamp;
        }
        Ok(())
    }

    pub fn id(&self) -> &UUID {
        &self.id
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
