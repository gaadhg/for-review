use super::*;

#[derive(Debug)]
pub enum CardsetError {
    Internal,
    IndexOutOfBounds,
    FlashcardError(FlashcardError),
    CardsetNotFound,
    InvalidName,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Cardset {
    id: UUIDv7,
    name: String,

    label: Subject,
    pub cards: Vec<Flashcard>,
}

impl Cardset {
    pub fn new(name: String, label: Subject, cards: Vec<Flashcard>) -> Result<Self, CardsetError> {
        let Ok(uuid) = UUIDv7::new() else {
            return Err(CardsetError::Internal);
        };
        let result = Self {
            id: uuid,
            name,
            label,
            cards,
        };
        Ok(result)
    }
    pub fn update_cardset(
        &mut self,
        name: Option<String>,
        label: Option<Subject>,
    ) -> Result<(), CardsetError> {
        if let Some(name) = name {
            self.name = name;
        }

        if let Some(label) = label {
            self.label = label;
        }

        Ok(())
    }

    pub fn id(&self) -> &UUIDv7 {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn label(&self) -> &Subject {
        &self.label
    }
}

impl From<FlashcardError> for CardsetError {
    fn from(error: FlashcardError) -> Self {
        CardsetError::FlashcardError(error)
    }
}
