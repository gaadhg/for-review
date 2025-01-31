use chrono::Weekday;

use super::*;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Schedule {
    data: [Vec<Subject>; 7]
}

impl Schedule {
    pub fn new() -> Self {
        Self {
            data: std::array::from_fn(|_| Vec::new())
        }
    }
    // Return the schedule for the whole week
    pub fn week(&self) -> &[Vec<Subject>; 7] {
        &self.data
    }

    // Return the data for a given day
    pub fn get_day(&self, day: Weekday) -> &[Subject]{
        &self.data[day.num_days_from_monday() as usize]
    }

    pub fn get_mut_day(&mut self, day: Weekday) -> &mut Vec<Subject> {
        &mut self.data[day.num_days_from_monday() as usize]
    }

    pub fn set_schedule(&mut self, schedule: [Vec<Subject>; 7]) {
        self.data = schedule;
    }
}

impl Schedule {
    pub fn from(subjects: [Vec<Subject>; 7]) -> Self {
        let mut schedule = Schedule::new();
        schedule.set_schedule(subjects);
        schedule
    }
}