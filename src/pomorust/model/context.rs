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
}
