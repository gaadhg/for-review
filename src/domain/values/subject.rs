use super::*;

#[derive(Debug)]
pub enum SubjectError {
    NameTooLong
}

#[derive(Clone, Hash, PartialEq, Eq, Debug, serde::Deserialize, serde::Serialize)]
pub struct Subject {
    color: Color,
    subject: String
}

impl Subject {
    pub fn new<S: Into<String>>(name: S, color: Color) -> Result<Self, SubjectError> {
        let name = name.into();
        if name.len() > 64 { return Err(SubjectError::NameTooLong) }
        Ok(Self {
            color: color,
            subject: name
        })
    }

    pub fn edit<S: Into<String>>(&mut self, name: Option<S>, color: Option<Color>) -> Result<(), SubjectError> {
        let name = name.map(|s| s.into());

        if let Some(name) = name {
            if name.len() > 64 { return Err(SubjectError::NameTooLong) }
            self.subject = name;
        }

        if let Some(color) = color {
            self.color = color;
        }
        Ok(())
    }
}