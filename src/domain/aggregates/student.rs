use std::{collections::HashSet, hash::Hash};

use chrono::{DateTime, Utc};
use indexmap::IndexSet;

use native_db::*;
use native_model::{native_model, Model};

use super::*;

#[derive(Debug)]
pub enum StudentError {
    InternalError,
    PasswordError(PasswordError),
    EmailError(EmailError),
    EventError(EventError),
    CardsetError(CardsetError)
}

#[derive(serde::Serialize, serde::Deserialize)]
#[native_model(id = 1, version = 1)]
#[native_db]
pub struct Student {
    #[primary_key]
    id: UUID,
    #[secondary_key]
    email: Email,
    password: Password,

    subjects: IndexSet<Subject>,
    schedule: Schedule,
    events: Vec<Event>,
    cardsets: Vec<Cardset>
}

impl Student {
    pub fn new(email: String, password: String, schedule: Option<[Vec<Subject>;7]>) -> Result<Self, StudentError> {
        let subjects: IndexSet<Subject> = schedule
        .as_ref()
        .map_or_else(IndexSet::new, |s| {
            s.iter().flat_map(|day| day.iter().cloned()).collect()
        });


        Ok(Self {
            id: UUID::new()?,
            email: Email::new(email)?,
            password: Password::new(password)?,
            
            subjects: subjects,
            // If schedule is None set empty schedule else if it is build a schedule out of the array
            schedule: Schedule::from(schedule.unwrap_or_default()),            
            events: vec![],
            cardsets: vec![]
        })
    }

    // █ █▀▄ 
    // █ █▄▀ 
    pub fn id(&self) -> &UUID {
        &self.id
    }

    // ██▀ █▄ ▄█ ▄▀▄ █ █   
    // █▄▄ █ ▀ █ █▀█ █ █▄▄ 
    
    pub fn email(&self) -> &Email {
        &self.email
    }

    pub fn change_email(&mut self, new_email: String, current_password: String) -> Result<(), StudentError> {
        if (!self.validate(current_password)?) { return Err(PasswordError::IncorrectPassword.into()) }
        self.email.change(new_email)?;
        Ok(())
    }

    // █▀▄ ▄▀▄ ▄▀▀ ▄▀▀ █   █ ▄▀▄ █▀▄ █▀▄ 
    // █▀  █▀█ ▄█▀ ▄█▀ ▀▄▀▄▀ ▀▄▀ █▀▄ █▄▀ 

    pub fn validate(&self, password: String) -> Result<bool, StudentError> {
        self.password.validate(password).map_err(|err| err.into())
    }

    pub fn change(&mut self, new_password: String, current_password: String) -> Result<(), StudentError> {
        if (!self.validate(current_password)?) { return Err(PasswordError::IncorrectPassword.into()) }

        self.password.change(new_password)?;

        Ok(())
    }

    
    // ▄▀▀ █ █ ██▄   █ ██▀ ▄▀▀ ▀█▀ ▄▀▀
    // ▄█▀ ▀▄█ █▄█ ▀▄█ █▄▄ ▀▄▄  █  ▄█▀

    // The subjects act as labels for events, flashcard sets and elements from the weekly schedule.
    // The logic here is if Student adds new subject to his schedule, it should become available as label for events
    // or vice-versa. If he adds it as a label for a Flashcard set for ex. He should be able to use it also as a subject in his schedule.
    // The subject (or label) list is Set because it needs to hold only unique values.
    pub fn subjects(&self) -> &IndexSet<Subject>{
        &self.subjects
    }

    // Internal function to check if subject is referenced in any event, flashcard set or the schedule and if not to remove it from the list.
    fn purge_subjects(&mut self) {
        self.subjects.retain(|s| {
            self.cardsets.iter().any(|set| set.label() == s)
                || self.events.iter().any(|event| event.label() == Some(s))
                || self.schedule.week().iter().any(|day| day.contains(s))
        });
    }

    fn add_subject(&mut self, subject: Subject) {
        self.subjects.insert(subject);
    }

    
    // ▄▀▀ ▄▀▀ █▄█ ██▀ █▀▄ █ █ █   ██▀ 
    // ▄█▀ ▀▄▄ █ █ █▄▄ █▄▀ ▀▄█ █▄▄ █▄▄ 

    pub fn schedule(&self) -> &Schedule {
        &self.schedule
    }

    pub fn change_schedule(&mut self, new_schedule: [Vec<Subject>; 7]) {
        // Add all subjects in the schedule in the subject list so they could be used to label events & etc...
        new_schedule.iter().flatten().for_each(|subject| self.add_subject(subject.clone()));

        self.schedule.set_schedule(new_schedule);
    }

    
    // ██▀ █ █ ██▀ █▄ █ ▀█▀ ▄▀▀ 
    // █▄▄ ▀▄▀ █▄▄ █ ▀█  █  ▄█▀

    pub fn schedule_event(&mut self, name: String, description: String, timestamp: DateTime<Utc>, label: Option<Subject>) -> Result<(), StudentError> {
        // Add the subject in the subject/label list so they could be in the schedule & etc...
        if let Some(subject) = label.clone() {
            self.add_subject(subject);
        }
        let event = Event::new(name, description, timestamp, label)?;

        self.events.push(event);
        Ok(())
    }
    pub fn events(&self) -> &[Event] {
        &self.events
    }

    pub fn cancel_event<ID: Into<u128> + Copy>(&mut self, event_id: ID) -> Result<(), StudentError> {
        
        let index = self.events.iter().position(|event| event.id().get() == event_id.into());
        match index {
            Some(index) => {
                self.events.remove(index);
                //Purge the subjects after removing event in case the event was the last element holding the subject
                self.purge_subjects();
                return Ok(());
            },
            None => return Err(EventError::EventNotFound.into())
        }
    }

    pub fn get_event<ID: Into<u128> + Copy>(&self, event_id: ID) -> Option<&Event> {
        // TODO: Do something about double reference maybe?
        self.events.iter().find(|event| event.id().get() == event_id.into())
    }
    // TODO: Maybe implement methods for getting events by date predicate? Date range?
    
    
    // ▄▀▀ ▄▀▄ █▀▄ █▀▄ ▄▀▀ ██▀ ▀█▀ ▄▀▀ 
    // ▀▄▄ █▀█ █▀▄ █▄▀ ▄█▀ █▄▄  █  ▄█▀

    pub fn cardsets(&self) -> &[Cardset] {
        &self.cardsets
    }

    pub fn new_cardset(&mut self, name: String, subject: Subject) -> Result<(), StudentError> {
        self.cardsets.push(Cardset::new(name, subject.clone(), vec![])?);
        self.add_subject(subject);
        Ok(())
    }

    pub fn add_card<ID: Into<u128> + Copy>(&mut self, cardset_id: ID, question: String, answer: String) -> Result<(), StudentError> {
        match self.cardsets.iter_mut().find(|set: &&mut Cardset| set.id().get() == cardset_id.into())
        {
            Some(cardset) => {
                cardset.add_card(question, answer)?;

                return Ok(());
            },
            None => return Err(CardsetError::CardsetNotFound.into())
        }
    }
    // TODO: More card operations
}

impl From<PasswordError> for StudentError {
    fn from(error: PasswordError) -> Self {
        match error {
            PasswordError::InternalError => StudentError::InternalError,
            _ => StudentError::PasswordError(error)
        }
    }
}

impl From<EmailError> for StudentError {
    fn from(error: EmailError) -> Self {
        match error {
            EmailError::InternalError => StudentError::InternalError,
            _ => StudentError::EmailError(error)
        }
    }
}

impl From<EventError> for StudentError {
    fn from(error: EventError) -> Self {
        match error {
            EventError::InternalError => StudentError::InternalError,
            _ => StudentError::EventError(error)
        }
    }
}

impl From<CardsetError> for StudentError {
    fn from(error: CardsetError) -> Self {
        match error {
            CardsetError::InternalError => StudentError::InternalError,
            _ => StudentError::CardsetError(error)
        }
    }
}


impl From<UUIDError> for StudentError {
    fn from(_: UUIDError) -> Self {
        StudentError::InternalError
    }
}



mod test {
    use std::{collections::HashSet, ops::Index};
    use indexmap::IndexSet;

    use crate::domain::*;
    #[test]
    pub fn student_new() {
        let email = "test@example.com".to_string();
        let password = "SecurePassword123".to_string();

        let schedule: [Vec<Subject>; 7] = [
            vec![Subject::new("Math", Color::White).unwrap(), Subject::new("English", Color::White).unwrap()],
            vec![Subject::new("Math", Color::White).unwrap()],
            vec![Subject::new("Math", Color::White).unwrap(), Subject::new("Japanese", Color::White).unwrap()],
            vec![Subject::new("Math", Color::White).unwrap()],
            vec![Subject::new("Math", Color::White).unwrap()],
            vec![Subject::new("Math", Color::White).unwrap(), Subject::new("Chem", Color::White).unwrap()],
            vec![Subject::new("Math", Color::White).unwrap()],
        ];

        let student = Student::new(email.clone(), password.clone(), Some(schedule)).unwrap();
        

        let mut asserted_hashset: IndexSet<Subject> = IndexSet::new();
        asserted_hashset.insert(Subject::new("Math", Color::White).unwrap());
        asserted_hashset.insert(Subject::new("Chem", Color::White).unwrap());
        asserted_hashset.insert(Subject::new("Japanese", Color::White).unwrap());
        asserted_hashset.insert(Subject::new("English", Color::White).unwrap());
        assert_eq!(student.subjects, asserted_hashset);
    }
}


