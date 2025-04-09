mod err;

use err::{ParseErr, ReadErr};
use std::{error::Error, fs, path::Path};

/// Represents a task in the todo list.
#[derive(Debug, Eq, PartialEq)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub level: u32,
}

/// Represents the todo list.
#[derive(Debug, Eq, PartialEq)]
pub struct TodoList {
    pub title: String,
    pub tasks: Vec<Task>,
}

impl TodoList {
    /// Reads and parses the todo list from the specified file path.
    pub fn get_todo<P: AsRef<Path>>(path: P) -> Result<TodoList, Box<dyn Error>> {
        // Read the file content
        let content = fs::read_to_string(&path).map_err(|e| Box::new(ReadErr { child_err: Box::new(e) }) as Box<dyn Error>)?;
        
        // Parse the JSON content
        let parsed: serde_json::Value = serde_json::from_str(&content)
            .map_err(|e| Box::new(ParseErr::Malformed(Box::new(e))) as Box<dyn Error>)?;
        
        // Extract the title
        let title = parsed["title"].as_str()
            .ok_or_else(|| Box::new(ParseErr::Malformed("Missing or invalid 'title' field".into())) as Box<dyn Error>)?
            .to_string();
        
        // Extract tasks
        let tasks_array = parsed["tasks"].as_array()
            .ok_or_else(|| Box::new(ParseErr::Malformed("Missing or invalid 'tasks' field".into())) as Box<dyn Error>)?;
        
        if tasks_array.is_empty() {
            return Err(Box::new(ParseErr::Empty));
        }
        
        let mut tasks = Vec::new();
        for task in tasks_array {
            let id = task["id"].as_u64()
                .ok_or_else(|| Box::new(ParseErr::Malformed("Invalid or missing 'id' in task".into())) as Box<dyn Error>)?
                as u32;
            let description = task["description"].as_str()
                .ok_or_else(|| Box::new(ParseErr::Malformed("Invalid or missing 'description' in task".into())) as Box<dyn Error>)?
                .to_string();
            let level = task["level"].as_u64()
                .ok_or_else(|| Box::new(ParseErr::Malformed("Invalid or missing 'level' in task".into())) as Box<dyn Error>)?
                as u32;
            
            tasks.push(Task { id, description, level });
        }
        
        Ok(TodoList { title, tasks })
    }
}
