use chrono;

use pomorust::model::tasks::Task;
use pomorust::utils::MaybeLocalDate;

pub struct Context {
    /// Should OS level notification be used
    pub use_notification: bool,
    /// Should sounds be played
    pub use_sound: bool,
    /// Listing of tasks recorded by the user
    pub tasks: Vec<Task>,
    /// When was the last pomodoro done ?
    pub last_pomodoro: MaybeLocalDate,
    /// How many sequential pomodori were run ?
    pub pomodori_count: u16
}

#[derive(Debug)]
pub enum IdentificationError {
    NoMatch,
    AmbiguousMatch
}

impl Context {
    pub fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    pub fn has_ongoing_task(&self) -> bool {
        let ongoings = self.tasks.iter().find(|&x| x.is_ongoing);
        ongoings.is_some()
    }

    /// Check if a given identifier identify one and only one
    /// task.
    pub fn is_valid_identifier(&self, identifier: &str) -> Result<(), IdentificationError> {
        let mut matching = self.tasks
            .iter()
            .filter(|&x| x.can_be_identified_by(&identifier));
        match matching.next() {
            None => Err(IdentificationError::NoMatch),
            Some(_) => match matching.next() {
                None => Ok(()),
                Some(_) => Err(IdentificationError::AmbiguousMatch)
            }
        }
    }

    pub fn get_task(&mut self, identifier: &str) -> &mut Task {
        self.tasks
            .iter_mut()
            .find(|x| x.can_be_identified_by(&identifier))
            .unwrap()
    }

    pub fn get_current_tasks(&self) -> Vec<&Task> {
        self.tasks.iter().filter(|&x| !x.is_finished()).collect::<Vec<&Task>>()
    }

    /// Context must manage a basic idea of the pomodoro technique: regular pauses.
    /// After each pomodori, one should take a 5 minute pauses.
    /// Every four pomodori, a longer pause should be taken.
    /// However,pomorust demands that the user regularly inputs on what task he's
    /// working, and the count of successive pomoodori could easily be wrong.
    /// So we will only count pomodori as successive if the last pomodoro count
    /// was less than 40 minutes ago.
    pub fn last_pomodoro_was_recent(&self) -> bool {
        let right_now = chrono::Local::now();
        match self.last_pomodoro {
            None => false,
            Some(t) => (right_now - t).num_minutes() <= 40
        }
    }

    pub fn increment_pomodoro_count(&mut self) {
        if self.last_pomodoro_was_recent() {
            self.pomodori_count += 1 ;
            if self.pomodori_count > 3 {
                self.pomodori_count = 0;
            }
        } else {
            self.pomodori_count = 1;
        }
        self.last_pomodoro = Some(chrono::Local::now());
    }

    pub fn should_be_long_pause(&self) -> bool {
        self.last_pomodoro_was_recent() && self.pomodori_count == 3
    }
}
