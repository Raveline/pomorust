use pomorust::model::tasks;

pub struct Context {
    pub tasks: Vec<tasks::Task>
}

impl Context {
    pub fn add_task(&mut self, task: tasks::Task) {
        self.tasks.push(task);
    }
}
