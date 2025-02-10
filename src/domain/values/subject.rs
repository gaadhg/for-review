use serde::{Deserialize, Serialize};
use smol_str::*;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SubjectError {
    #[error("name too long")]
    NameTooLong,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct Subject {
    label: Color,
    name: SmolStr,
}

impl Subject {
    pub(in crate::domain) fn new(name: &str, label: Color) -> Result<Self, SubjectError> {
        if name.len() > 64 {
            return Err(SubjectError::NameTooLong);
        }
        Ok(Self {
            label,
            name: name.to_smolstr(),
        })
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn label(&self) -> Color {
        self.label
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum Color {
    White,
    Black,
}
