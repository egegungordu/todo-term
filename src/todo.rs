use crate::todo_serializer::TodoSerializer;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
    complete_tasks: Vec<String>,
    incomplete_tasks: Vec<String>,
    #[serde(skip)]
    serializer: Option<Box<dyn TodoSerializer>>,
}

impl Todo {
    pub fn new() -> Todo {
        Todo {
            complete_tasks: Vec::new(),
            incomplete_tasks: Vec::new(),
            serializer: None,
        }
    }

    pub fn with_serializer(serializer: Box<dyn TodoSerializer>) -> Todo {
        let mut todo = Todo::new();
        todo.serializer = Some(serializer);
        todo
    }

    pub fn add_task(&mut self, index: usize, task: String) {
        assert!(index <= self.incomplete_tasks.len());

        self.incomplete_tasks.insert(index, task);
    }

    pub fn toggle_task(&mut self, index: usize) {
        assert!(index < self.len());

        if index < self.incomplete_tasks.len() {
            let task = self.incomplete_tasks.remove(index);
            self.complete_tasks.insert(0, task);
        } else {
            let task = self
                .complete_tasks
                .remove(index - self.incomplete_tasks.len());
            self.incomplete_tasks.push(task);
        }
    }

    pub fn delete_task(&mut self, index: usize) {
        if index < self.incomplete_tasks.len() {
            self.incomplete_tasks.remove(index);
        } else {
            self.complete_tasks
                .remove(index - self.incomplete_tasks.len());
        }
    }

    pub fn get_complete_tasks(&self) -> &Vec<String> {
        &self.complete_tasks
    }

    pub fn get_incomplete_tasks(&self) -> &Vec<String> {
        &self.incomplete_tasks
    }

    pub fn edit_task(&mut self, index: usize, task: String) {
        assert!(index < self.len());

        if index < self.incomplete_tasks.len() {
            self.incomplete_tasks[index] = task;
        } else {
            self.complete_tasks[index - self.incomplete_tasks.len()] = task;
        }
    }

    pub fn len(&self) -> usize {
        self.incomplete_tasks.len() + self.complete_tasks.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(serializer) = &self.serializer {
            serializer.save(self)?;
        }
        Ok(())
    }

    pub fn load(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(serializer) = &self.serializer {
            let todo = serializer.load()?;
            self.complete_tasks = todo.complete_tasks;
            self.incomplete_tasks = todo.incomplete_tasks;
        }
        Ok(())
    }
}
