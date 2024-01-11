use crate::todo::Todo;
use std::error::Error;

mod json_serializer;

pub use json_serializer::JsonSerializer;

pub trait TodoSerializer {
    fn save(&self, todo: &Todo) -> Result<(), Box<dyn Error>>;
    fn load(&self) -> Result<Todo, Box<dyn Error>>;
}

impl std::fmt::Debug for dyn TodoSerializer + 'static {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TodoSerializer").finish()
    }
}
