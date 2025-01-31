#[derive(Debug)]
pub enum FlashcardError {
    QuestionTooLong,
    AnswerTooLong,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Flashcard {
    question: String,
    answer: String
}

impl Flashcard {
    pub fn new(question: String, answer: String) -> Result<Self, FlashcardError> {
        if question.len() > 256 { return Err(FlashcardError::QuestionTooLong) }
        if answer.len() > 512 { return Err(FlashcardError::AnswerTooLong) }
        Ok(Self {
            question: question,
            answer: answer
        })
    }

    pub fn edit(&mut self, question: String, answer: String) -> Result<(), FlashcardError> {
        if question.len() > 256 { return Err(FlashcardError::QuestionTooLong) }
        if answer.len() > 512 { return Err(FlashcardError::AnswerTooLong) }

        self.question = question;
        self.answer = answer;
        Ok(())
    }
}