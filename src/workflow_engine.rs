pub struct Workflow {
    tasks: Vec<Box<dyn Fn()>>,
}

impl Workflow {
#[allow(dead_code)]
    pub fn new() -> Self {
        Workflow { tasks: vec![] }
    }

#[allow(dead_code)]
    pub fn add_task<F>(&mut self, task: F)
    where
        F: Fn() + 'static,
    {
        self.tasks.push(Box::new(task));
    }

#[allow(dead_code)]
    pub fn execute(&self) {
        for task in &self.tasks {
            task();
        }
    }
}

