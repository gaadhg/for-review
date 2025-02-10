use std::error;

use serde::{Deserialize, Serialize};
use smol_str::{SmolStr, ToSmolStr};
use thiserror::Error;

use crate::domain::*;

#[derive(Debug, Error)]
pub enum CardsetError {
    #[error("name over 64 characters")]
    NameTooLong,
    #[error("flashcard error: `{0}`")]
    FlashcardError(#[from] FlashcardError),
    #[error("index out of bounds")]
    IndexOutOfBounds,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Cardset {
    id: UUIDv7,
    name: SmolStr,
    label: Subject,
    cards: Vec<Flashcard>,
}

impl Cardset {
    pub(in crate::domain) fn new(
        id: UUIDv7,
        name: &str,
        label: Subject,
    ) -> Result<Self, CardsetError> {
        if name.len() > 64 {
            return Err(CardsetError::NameTooLong);
        }
        Ok(Self {
            id,
            name: name.to_smolstr(),
            label,
            cards: vec![],
        })
    }

    pub fn id(&self) -> UUIDv7 {
        self.id.clone()
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn label(&self) -> &Subject {
        &self.label
    }
    //
    pub fn cards(&self) -> &[Flashcard] {
        &self.cards
    }

    pub fn card(&self, index: usize) -> Result<&Flashcard, CardsetError> {
        if index < self.cards.len() {
            return Ok(self.cards.get(index).unwrap());
        }
        Err(CardsetError::IndexOutOfBounds)
    }

    pub fn add_card(&mut self, question: &str, answer: &str) -> Result<usize, CardsetError> {
        self.cards.push(Flashcard::new(question, answer)?);
        Ok(self.cards.len() - 1)
    }

    pub fn edit_card(
        &mut self,
        index: usize,
        question: &str,
        answer: &str,
    ) -> Result<usize, CardsetError> {
        if index < self.cards.len() {
            self.cards[index] = Flashcard::new(question, answer)?;
            return Ok(index);
        }

        Err(CardsetError::IndexOutOfBounds)
    }

    pub fn remove_card(&mut self, index: usize) -> Result<(), CardsetError> {
        if index < self.cards.len() {
            self.cards.remove(index);
            return Ok(());
        }
        Err(CardsetError::IndexOutOfBounds)
    }
}
