pub struct Rustochitelstvo {
    tasks: Vec<Task>,
}

type Task = Box<dyn FnOnce() -> anyhow::Result<()>>;

impl Rustochitelstvo {
    pub fn new() -> Self {
        Self { tasks: vec![] }
    }

    pub fn add_tasks(&mut self, tasks: impl Iterator<Item = Task>) {
        for task in tasks {
            self.tasks.push(task);
        }
    }
}
