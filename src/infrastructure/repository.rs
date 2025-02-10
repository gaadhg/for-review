use std::sync::Arc;

use axum_login::{AuthnBackend, UserId};
use native_db::{Builder, Database, Models};
use once_cell::sync::Lazy;
use thiserror::Error;
use tokio::task;

use crate::domain::*;

static MODELS: Lazy<Models> = Lazy::new(|| {
    let mut models = Models::new();
    models.define::<Student>().unwrap();
    models
});

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("fatal database error: `{0}`")]
    DatabaseError(#[from] native_db::db_type::Error),
    // #[error("invalid id key: `{0}`")]
    // InvalidUUID(#[from] UUIDError),
    #[error("invalid email key: `{0}`")]
    InvalidEmail(#[from] EmailError),
}

#[derive(Clone)]
pub struct StudentRepository {
    db: Arc<Database<'static>>,
}

impl StudentRepository {
    pub fn new() -> Result<Self, RepositoryError> {
        let db = Builder::new().create_in_memory(&MODELS)?;
        Ok(Self { db: Arc::new(db) })
    }

    pub fn get_student_by_id(&self, id: UUIDv7) -> Result<Option<Student>, RepositoryError> {
        // let id = UUIDv7::from_u128(id);
        Ok(self.db.r_transaction()?.get().primary(id)?)
    }

    pub fn get_student_by_email(&self, email: &str) -> Result<Option<Student>, RepositoryError> {
        let email = Email::new(email)?;
        Ok(self
            .db
            .r_transaction()?
            .get()
            .secondary(StudentKey::email, email)?)
    }

    pub fn update_student(&mut self, new: Student) -> Result<(), RepositoryError> {
        let rw = self.db.rw_transaction()?;
        rw.upsert(new)?;
        rw.commit()?;
        Ok(())
    }
}

#[async_trait::async_trait]
impl AuthnBackend for StudentRepository {
    type User = Student;
    type Credentials = super::Credentials;
    type Error = RepositoryError;

    async fn authenticate(
        &self,
        creds: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        let student = self.get_student_by_email(&creds.email)?;
        match student {
            Some(student) => {
                if student.validate_password(&creds.password) {
                    return Ok(Some(student));
                } else {
                    return Ok(None);
                }
            }
            None => {
                return Ok(None);
            }
        }
    }

    async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
        let user = self.get_student_by_id(user_id.clone())?;

        Ok(user)
    }
}

pub type AuthSession = axum_login::AuthSession<StudentRepository>;
