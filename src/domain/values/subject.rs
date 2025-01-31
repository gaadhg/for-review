use super::*;

#[derive(Debug)]
pub enum SubjectError {
    NameTooLong,
}

#[derive(Clone, Hash, PartialEq, Eq, Debug, serde::Deserialize, serde::Serialize)]
pub struct Subject {
    color: Color,
    subject: String,
}

impl Subject {
    pub fn new(name: String, color: Color) -> Result<Self, SubjectError> {
        if name.len() > 64 {
            return Err(SubjectError::NameTooLong);
        }
        Ok(Self {
            color,
            subject: name,
        })
    }

    pub fn set_name(&mut self, name: String) -> Result<(), SubjectError> {
        if name.len() > 64 {
            return Err(SubjectError::NameTooLong);
        }
        self.subject = name;
        Ok(())
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }
    // pub fn edit<S: Into<String>>(&mut self, name: Option<S>, color: Option<Color>) -> Result<(), SubjectError> {
    //     let name = name.map(|s| s.into());

    //     if let Some(name) = name {
    //         if name.len() > 64 { return Err(SubjectError::NameTooLong) }
    //         self.subject = name;
    //     }

    //     if let Some(color) = color {
    //         self.color = color;
    //     }
    //     Ok(())
    // }
}
