use super::TodoSerializer;

pub struct JsonSerializer {
    path: String,
}

impl JsonSerializer {
    pub fn new(path: String) -> JsonSerializer {
        JsonSerializer { path }
    }
}

impl TodoSerializer for JsonSerializer {
    fn save(&self, todo: &crate::todo::Todo) -> Result<(), Box<dyn std::error::Error>> {
        let contents = serde_json::to_string_pretty(todo)?;
        std::fs::write(&self.path, contents)?;
        Ok(())
    }

    fn load(&self) -> Result<crate::todo::Todo, Box<dyn std::error::Error>> {
        let contents = std::fs::read_to_string(&self.path)?;
        let todo = serde_json::from_str(&contents)?;
        Ok(todo)
    }
}

