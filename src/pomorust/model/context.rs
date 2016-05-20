use pomorust::model::tasks::Task;

pub struct Context {
    pub use_notification: bool,
    pub use_sound: bool,
    pub tasks: Vec<Task>
}

impl Context {
    pub fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    pub fn has_ongoing_task(&self) -> bool {
        let ongoings = self.tasks.iter().find(|&x| x.is_ongoing);
        ongoings.is_some()
    }
}
