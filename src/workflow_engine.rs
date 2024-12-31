pub struct Workflow {
    tasks: Vec<Box<dyn Fn()>>,
}

impl Workflow {
    pub fn new() -> Self {
        Workflow { tasks: vec![] }
    }

    pub fn add_task<F>(&mut self, task: F)
    where
        F: Fn() + 'static,
    {
        self.tasks.push(Box::new(task));
    }

    pub fn execute(&self) {
        for task in &self.tasks {
            task();
        }
    }
}

