use pomorust::model::tasks::Task;

pub struct Context {
    pub tasks: Vec<Task>
}

impl Context {
    pub fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }
}
