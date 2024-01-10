use std::error;

use ratatui::widgets::ListState;

use crate::todo::Todo;

pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

pub struct App {
    running: bool,
    show_help: bool,
    todo: Todo,
    todo_list_state: ListState,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            show_help: false,
            todo: Todo::new(),
            todo_list_state: ListState::default(),
        }
    }
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn tick(&self) {}

    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn add_task_above(&mut self) {
        let new_task_index = {
            if self.todo.is_empty() {
                0
            } else {
                match self.todo_list_state.selected() {
                    Some(i) => std::cmp::min(i, self.todo.get_incomplete_tasks().len()),
                    None => 0,
                }
            }
        };
        self.todo_list_state.select(Some(new_task_index));
        self.todo.add_task(
            new_task_index,
            String::from(format!("Task {}", new_task_index)),
        );
    }

    pub fn add_task_below(&mut self) {
        let new_task_index = {
            if self.todo.is_empty() {
                0
            } else {
                match self.todo_list_state.selected() {
                    Some(i) => std::cmp::min(i + 1, self.todo.get_incomplete_tasks().len()),
                    None => self.todo.len(),
                }
            }
        };
        self.todo_list_state.select(Some(new_task_index));
        self.todo.add_task(
            new_task_index,
            String::from(format!("Task {}", new_task_index)),
        );
    }

    pub fn delete_task(&mut self) {
        if self.todo.is_empty() {
            return;
        }

        if let Some(i) = self.todo_list_state.selected() {
            self.todo.delete_task(i);
            if i >= self.todo.len() {
                self.select_last_task();
            }
        }
    }

    pub fn change_task(&mut self) {
        todo!();
    }

    pub fn toggle_task(&mut self) {
        match self.todo_list_state.selected() {
            Some(i) => {
                self.todo.toggle_task(i);
            }
            None => {}
        }
    }

    pub fn navigate_down(&mut self) {
        if let Some(i) = self.todo_list_state.selected() {
            let next_index = if i >= self.todo.len() - 1 { 0 } else { i + 1 };
            self.todo_list_state.select(Some(next_index));
        };
    }

    pub fn navigate_up(&mut self) {
        if let Some(i) = self.todo_list_state.selected() {
            let next_index = if i == 0 { self.todo.len() - 1 } else { i - 1 };
            self.todo_list_state.select(Some(next_index));
        };
    }

    pub fn navigate_top(&mut self) {
        self.todo_list_state.select(Some(0));
    }

    pub fn navigate_bottom(&mut self) {
        self.todo_list_state.select(Some(self.todo.len() - 1));
    }

    pub fn toggle_help(&mut self) {
        self.show_help = !self.show_help;
    }

    pub fn is_help_visible(&self) -> bool {
        self.show_help
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn is_todo_empty(&self) -> bool {
        self.todo.is_empty()
    }

    pub fn get_todo_list_state(&mut self) -> &mut ListState {
        &mut self.todo_list_state
    }

    pub fn get_complete_tasks(&self) -> &Vec<String> {
        self.todo.get_complete_tasks()
    }

    pub fn get_incomplete_tasks(&self) -> &Vec<String> {
        self.todo.get_incomplete_tasks()
    }

    fn select_last_task(&mut self) {
        let selected = match self.todo.len() {
            0 => None,
            n => Some(n - 1),
        };
        self.todo_list_state.select(selected);
    }
}
