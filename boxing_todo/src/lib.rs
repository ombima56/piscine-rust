mod err;
pub use err::{ParseErr, ReadErr};

use std::error::Error;
use std::{fs::File, io::Read};
extern crate json;

#[derive(Debug, Eq, PartialEq)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub level: u32,
}

#[derive(Debug, Eq, PartialEq)]
pub struct TodoList {
    pub title: String,
    pub tasks: Vec<Task>,
}

impl TodoList {
    pub fn get_todo(path: &str) -> Result<TodoList, Box<dyn Error>> {
        let file_result = File::open(path);
        let mut file = match file_result {
            Ok(f) => f,
            Err(e) => {
                let read_err = ReadErr {
                    child_err: Box::new(e),
                };
                return Err(Box::new(read_err));
            }
        };
        
        let mut content = String::new();
        match file.read_to_string(&mut content) {
            Ok(_) => {},
            Err(e) => {
                let read_err = ReadErr {
                    child_err: Box::new(e),
                };
                return Err(Box::new(read_err));
            }
        }
        
        if content.trim().is_empty() {
            return Err(Box::new(ParseErr::Empty));
        }
        
        let parsed_json = match json::parse(&content) {
            Ok(json) => json,
            Err(e) => {
                return Err(Box::new(ParseErr::Malformed(Box::new(e))));
            }
        };
        
        let title = match parsed_json["title"].as_str() {
            Some(t) => t.to_string(),
            None => return Err(Box::new(ParseErr::Empty)),
        };

        if parsed_json["tasks"].len() == 0 {
            return Err(Box::new(ParseErr::Empty));
        }

        let mut tasks = Vec::new();
        for i in 0..parsed_json["tasks"].len() {
            let id = match parsed_json["tasks"][i]["id"].as_u32() {
                Some(id) => id,
                None => return Err(Box::new(ParseErr::Empty)),
            };
            
            let description = match parsed_json["tasks"][i]["description"].as_str() {
                Some(desc) => desc.to_string(),
                None => return Err(Box::new(ParseErr::Empty)),
            };
            
            let level = match parsed_json["tasks"][i]["level"].as_u32() {
                Some(lvl) => lvl,
                None => return Err(Box::new(ParseErr::Empty)),
            };
            
            tasks.push(Task {
                id,
                description,
                level,
            });
        }
        
        Ok(TodoList { title, tasks })
    }
}
