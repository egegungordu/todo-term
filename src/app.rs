use std::error;

use ratatui::widgets::ListState;

use crate::{action_display::ActionDisplay, todo::Todo, todo_serializer::JsonSerializer};

use std::fmt;

pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AppMode {
    Normal,
    Insert,
}

impl fmt::Display for AppMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppMode::Normal => write!(f, "Normal"),
            AppMode::Insert => write!(f, "Insert"),
        }
    }
}

pub struct App {
    running: bool,
    show_help: bool,
    todo: Todo,
    action_display: ActionDisplay,
    todo_list_state: ListState,
    yank_buffer: Option<String>,
    mode: AppMode,
}

impl Default for App {
    fn default() -> Self {
        let mut action_display = ActionDisplay::new();
        let mut todo_list_state = ListState::default();
        let mut todo =
            Todo::with_serializer(Box::new(JsonSerializer::new("todo.json".to_string())));
        if let Err(e) = todo.load() {
            action_display.set(&format!("Error loading todo: {}", e));
        } else {
            todo_list_state.select(Some(0));
        }
        Self {
            running: true,
            show_help: false,
            todo,
            action_display,
            todo_list_state,
            yank_buffer: None,
            mode: AppMode::Normal,
        }
    }
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn tick(&mut self) {
        self.action_display.tick();
    }

    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn add_task_above(&mut self) {
        self.action_display.set("Added task above");

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
        self.todo.add_task(new_task_index, "".to_string());
    }

    pub fn add_task_below(&mut self) {
        self.action_display.set("Added task below");

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
        self.todo.add_task(new_task_index, "".to_string());
    }

    pub fn delete_task(&mut self) {
        self.action_display.set("Deleted task");

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

    pub fn enter_insert_mode(&mut self) {
        self.mode = AppMode::Insert;
    }

    pub fn exit_insert_mode(&mut self) {
        self.action_display.set("Saving todo");

        self.mode = AppMode::Normal;
    }

    pub fn append_to_task(&mut self, c: char) {
        if let Some(i) = self.todo_list_state.selected() {
            let task = if i < self.todo.get_incomplete_tasks().len() {
                self.todo.get_incomplete_tasks()[i].clone()
            } else {
                self.todo.get_complete_tasks()[i - self.todo.get_incomplete_tasks().len()].clone()
            };

            let new_task = format!("{}{}", task, c);

            self.todo.edit_task(i, new_task);
        }
    }

    pub fn pop_from_task(&mut self) {
        if let Some(i) = self.todo_list_state.selected() {
            let task = if i < self.todo.get_incomplete_tasks().len() {
                self.todo.get_incomplete_tasks()[i].clone()
            } else {
                self.todo.get_complete_tasks()[i - self.todo.get_incomplete_tasks().len()].clone()
            };

            let new_task = if task.len() > 0 {
                task[..task.len() - 1].to_string()
            } else {
                task
            };

            self.todo.edit_task(i, new_task);
        }
    }

    pub fn reset_task(&mut self) {
        if let Some(i) = self.todo_list_state.selected() {
            self.todo.edit_task(i, "".to_string());
        }
    }

    pub fn toggle_task(&mut self) {
        self.action_display.set("Toggled task");

        match self.todo_list_state.selected() {
            Some(i) => {
                self.todo.toggle_task(i);
            }
            None => {}
        }
    }

    pub fn yank_task(&mut self) {
        self.action_display.set("Yanked task");

        if let Some(i) = self.todo_list_state.selected() {
            let task = if i < self.todo.get_incomplete_tasks().len() {
                self.todo.get_incomplete_tasks()[i].clone()
            } else {
                self.todo.get_complete_tasks()[i - self.todo.get_incomplete_tasks().len()].clone()
            };

            self.yank_buffer = Some(task);
        }
    }

    pub fn paste_task_above(&mut self) {
        self.action_display.set("Pasted task above");

        if self.yank_buffer.is_none() {
            return;
        }

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
        self.todo
            .add_task(new_task_index, self.yank_buffer.clone().unwrap());
    }

    pub fn paste_task_below(&mut self) {
        self.action_display.set("Pasted task below");

        if self.yank_buffer.is_none() {
            return;
        }

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
        self.todo
            .add_task(new_task_index, self.yank_buffer.clone().unwrap());
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
        let next_selected = match self.todo.len() {
            0 => None,
            n => Some(n - 1),
        };
        self.todo_list_state.select(next_selected);
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

    pub fn get_mode(&self) -> AppMode {
        self.mode
    }

    pub fn get_action(&self) -> &str {
        self.action_display.get()
    }

    pub fn save(&mut self) {
        if let Err(e) = self.todo.save() {
            self.action_display.set(&format!("Error saving todo: {}", e));
        }
    }

    fn select_last_task(&mut self) {
        let selected = match self.todo.len() {
            0 => None,
            n => Some(n - 1),
        };
        self.todo_list_state.select(selected);
    }
}
