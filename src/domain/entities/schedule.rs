use crate::domain::*;
use arrayvec::ArrayVec;
use serde::{Deserialize, Serialize};

const MAX_SUBJECTS: usize = 8;
pub type Day = ArrayVec<Subject, MAX_SUBJECTS>;

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct Schedule {
    pub(in crate::domain) week: [Day; 7],
}

impl Schedule {
    pub(in crate::domain) fn new() -> Self {
        Self {
            week: [const { ArrayVec::new_const() }; 7],
        }
    }

    pub(in crate::domain) fn from(subjects: [Day; 7]) -> Self {
        Self { week: subjects }
    }
}
