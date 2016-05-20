use pomorust::model::tasks::Task;

pub struct Context {
    pub use_notification: bool,
    pub use_sound: bool,
    pub tasks: Vec<Task>
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
}
