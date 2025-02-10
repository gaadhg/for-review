use serde::{Deserialize, Serialize};
use smol_str::{SmolStr, ToSmolStr};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum FlashcardError {
    #[error("question over 256 characters")]
    QuestionTooLong,
    #[error("answer over 256 characters")]
    AnswerTooLong,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Flashcard {
    question: SmolStr,
    answer: SmolStr,
}

impl Flashcard {
    pub(in crate::domain) fn new(question: &str, answer: &str) -> Result<Self, FlashcardError> {
        if question.len() > 256 {
            return Err(FlashcardError::QuestionTooLong);
        }
        if answer.len() > 512 {
            return Err(FlashcardError::AnswerTooLong);
        }
        Ok(Self {
            question: question.to_smolstr(),
            answer: answer.to_smolstr(),
        })
    }

    pub fn question(&self) -> &str {
        &self.question
    }

    pub fn answer(&self) -> &str {
        &self.answer
    }
}
