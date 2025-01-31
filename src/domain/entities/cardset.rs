use super::*;

#[derive(Debug)]
pub enum CardsetError {
    InternalError,
    IndexOutOfBounds,
    FlashcardError(FlashcardError),
    CardsetNotFound,
    InvalidName
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Cardset {
    id: UUID,
    name: String,

    label: Subject,
    cards: Vec<Flashcard>
}

impl Cardset {
    pub fn new(name: String, label: Subject, cards: Vec<Flashcard>) -> Result<Self, CardsetError> {
        let Ok(uuid) = UUID::new() else { return Err(CardsetError::InternalError) };
        let result = Self {
            id: uuid,
            name: name,
            label: label,
            cards: cards
        };
        Ok(result)
    }
    pub fn update_cardset(&mut self, name: Option<String>, label: Option<Subject>) -> Result<(), CardsetError> {
        if let Some(name) = name {
            self.name = name;
        }

        if let Some(label) = label {
            self.label = label;
        }

        Ok(())
    }

    pub fn id(&self) -> &UUID {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn label(&self) -> &Subject {
        &self.label
    }

// Cards
    pub fn add_card(&mut self, question: String, answer: String) -> Result<(), CardsetError> {
        self.cards.push(Flashcard::new(question, answer)?);
        Ok(())
    }

    pub fn remove_at(&mut self, index: usize) -> Result<(), CardsetError> {
        if index < self.cards.len() {
            self.cards.remove(index);
            return Ok(());
        } else {
            return Err(CardsetError::IndexOutOfBounds)
        }
    }

    pub fn replace_at(&mut self, index: usize, new_card: Flashcard) -> Result<(), CardsetError> {
        if index < self.cards.len() {
            self.cards[index] = new_card;
            return Ok(());
        } else {
            return Err(CardsetError::IndexOutOfBounds)
        }
    }

    pub fn get_cards(&self) -> &[Flashcard] {
        &self.cards
    }

    pub fn get_mut_cards(&mut self) -> &mut [Flashcard] {
        &mut self.cards
    }

}

impl From<FlashcardError> for CardsetError {
    fn from(error: FlashcardError) -> Self {
        match error {
            _ => CardsetError::FlashcardError(error)
        }
    }
}