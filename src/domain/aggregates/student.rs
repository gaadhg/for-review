use axum_login::AuthUser;
use chrono::{DateTime, Utc, Weekday};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use vecmap::{VecSet, vecset};

use native_db::*;
use native_model::{Model, native_model};

use crate::domain::*;

#[derive(Debug, Error)]
pub enum StudentError {
    #[allow(clippy::upper_case_acronyms)]
    #[error("UUID Error: `{0}`")]
    UUID(#[from] UUIDError),
    #[error("email error: `{0}`")]
    Email(#[from] EmailError),
    #[error("password error: `{0}`")]
    Password(#[from] PasswordError),
    #[error("event error: `{0}`")]
    Event(#[from] EventError),
    #[error("cardset error: `{0}`")]
    Cardset(#[from] CardsetError),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[native_model(id = 1, version = 1)]
#[native_db]
pub struct Student {
    #[primary_key]
    id: UUIDv7,

    #[secondary_key(unique)]
    email: Email,
    password: Password,

    subjects: VecSet<Subject>,
    schedule: Schedule,
    events: Vec<Event>,
    cardsets: Vec<Cardset>,
}

impl Student {
    pub fn new(
        email: &str,
        password: &str,
        schedule: Option<[Day; 7]>,
    ) -> Result<Self, StudentError> {
        let mut subjects: VecSet<Subject> = vecset![];

        schedule
            .clone()
            .unwrap_or_default()
            .into_iter()
            .flatten()
            .for_each(|subject| {
                subjects.insert(subject);
            });
        Ok(Self {
            id: UUIDv7::new()?,
            email: Email::new(email)?,
            password: Password::new(password)?,

            subjects,
            events: vec![],
            schedule: Schedule::from(schedule.unwrap_or_default()),
            cardsets: vec![],
        })
    }

    pub fn id(&self) -> UUIDv7 {
        self.id.clone()
    }

    pub fn email(&self) -> &Email {
        &self.email
    }

    pub fn change_email(
        &mut self,
        new_email: &str,
        current_password: &str,
    ) -> Result<(), StudentError> {
        if self.validate_password(current_password) {
            self.email = Email::new(new_email)?;
            return Ok(());
        }
        Err(PasswordError::IncorrectPassword.into())
    }

    pub fn validate_password(&self, password: &str) -> bool {
        self.password.validate(password)
    }

    pub fn change_password(
        &mut self,
        new_password: &str,
        current_password: &str,
    ) -> Result<(), StudentError> {
        if self.validate_password(current_password) {
            self.password = Password::new(new_password)?;
            return Ok(());
        }
        Err(PasswordError::IncorrectPassword.into())
    }

    //
    pub fn subjects(&self) -> &[Subject] {
        self.subjects.as_slice()
    }

    fn add_subject(&mut self, subject: Subject) {
        self.subjects.insert(subject);
    }

    fn purge_subjects(&mut self) {
        self.subjects.retain(|s| {
            self.cardsets.iter().any(|set| set.label() == s)
                || self.events.iter().any(|event| event.label() == Some(s))
                || self.schedule.week.iter().any(|day| day.contains(s))
        });
    }

    //
    pub fn schedule(&self) -> &[Day; 7] {
        &self.schedule.week
    }

    pub fn schedule_day(&self, day: Weekday) -> &[Subject] {
        &self.schedule.week[day.num_days_from_monday() as usize]
    }

    pub fn set_schedule(&mut self, new: [Day; 7]) {
        new.iter()
            .flatten()
            .for_each(|subject| self.add_subject(subject.clone()));

        self.schedule.week = new;

        self.purge_subjects();
    }
    //

    pub fn events(&self) -> &[Event] {
        &self.events
    }

    pub fn event(&self, event_id: UUIDv7) -> Option<&Event> {
        self.events.iter().find(|event| event.id() == event_id)
    }

    pub fn schedule_event(
        &mut self,
        name: &str,
        description: &str,
        label: Option<Subject>,
        timestamp: DateTime<Utc>,
    ) -> Result<UUIDv7, StudentError> {
        let event_id = UUIDv7::new()?;
        let event = Event::new(
            event_id.clone(),
            name,
            description,
            label.clone(),
            timestamp,
        )?;
        self.events.push(event);

        if let Some(subject) = label {
            self.subjects.insert(subject);
        }

        Ok(event_id)
    }

    pub fn cancel_event(&mut self, event_id: UUIDv7) {
        let index = self.events.iter().position(|event| event.id() == event_id);
        if let Some(index) = index {
            self.events.remove(index);
            self.purge_subjects();
        }
    }
    //

    pub fn cardsets(&self) -> &[Cardset] {
        &self.cardsets
    }

    pub fn cardset(&self, cardset_id: UUIDv7) -> Option<&Cardset> {
        self.cardsets
            .iter()
            .find(|cardset| cardset.id() == cardset_id)
    }

    pub fn cardset_mut(&mut self, cardset_id: UUIDv7) -> Option<&mut Cardset> {
        self.cardsets
            .iter_mut()
            .find(|cardset| cardset.id() == cardset_id)
    }

    pub fn new_cardset(&mut self, name: &str, label: Subject) -> Result<UUIDv7, StudentError> {
        let cardset_id = UUIDv7::new()?;
        let cardset = Cardset::new(cardset_id.clone(), name, label.clone())?;
        self.cardsets.push(cardset);
        self.add_subject(label);
        Ok(cardset_id)
    }

    pub fn delete_cardset(&mut self, cardset_id: UUIDv7) {
        let index = self
            .cardsets
            .iter()
            .position(|cardset| cardset.id() == cardset_id);
        if let Some(index) = index {
            self.cardsets.remove(index);
            self.purge_subjects();
        }
    }
}

impl AuthUser for Student {
    type Id = UUIDv7;

    fn id(&self) -> Self::Id {
        self.id.clone()
    }

    fn session_auth_hash(&self) -> &[u8] {
        self.password.as_bytes()
    }
}
