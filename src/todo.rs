pub struct Todo {
    complete_tasks: Vec<String>,
    incomplete_tasks: Vec<String>,
}

impl Todo {
    pub fn new() -> Todo {
        Todo {
            complete_tasks: Vec::new(),
            incomplete_tasks: Vec::new(),
        }
    }

    pub fn add_task(&mut self, index: usize, task: String) {
        assert!(index <= self.incomplete_tasks.len());
        self.incomplete_tasks.insert(index, task);
    }

    pub fn toggle_task(&mut self, index: usize) {
        if index < self.incomplete_tasks.len() {
            let task = self.incomplete_tasks.remove(index);
            self.complete_tasks.push(task);
        } else {
            let task = self.complete_tasks.remove(index - self.incomplete_tasks.len());
            self.incomplete_tasks.push(task);
        }
    }

    pub fn delete_task(&mut self, index: usize) {
        if index < self.incomplete_tasks.len() {
            self.incomplete_tasks.remove(index);
        } else {
            self.complete_tasks.remove(index - self.incomplete_tasks.len());
        }
    }

    pub fn get_complete_tasks(&self) -> &Vec<String> {
        &self.complete_tasks
    }

    pub fn get_incomplete_tasks(&self) -> &Vec<String> {
        &self.incomplete_tasks
    }

    pub fn len(&self) -> usize {
        self.incomplete_tasks.len() + self.complete_tasks.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
