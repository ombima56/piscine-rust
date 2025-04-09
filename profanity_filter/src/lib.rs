pub struct Message {
    content: String,
    user: String,
}

impl Message {
    pub fn new(content: String, user: String) -> Self {
        Message { content, user }
    }

    pub fn send_ms(&self) -> Option<&str> {
        if self.content.is_empty() || self.content.to_lowercase().contains("stupid") {
            None
        } else {
            Some(&self.content)
        }
    }
}

pub fn check_ms(message: &str) -> Result<&str, &str> {
    let msg = Message::new(message.to_string(), "user".to_string());
    
    match msg.send_ms() {
        Some(_) => Ok(message),
        None => Err("ERROR: illegal"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_message() {
        let result = check_ms("hello there");
        assert_eq!(result, Ok("hello there"));
    }

    #[test]
    fn test_empty_message() {
        let result = check_ms("");
        assert_eq!(result, Err("ERROR: illegal"));
    }

    #[test]
    fn test_message_with_stupid() {
        let result = check_ms("you are stupid");
        assert_eq!(result, Err("ERROR: illegal"));
    }

    #[test]
    fn test_just_stupid() {
        let result = check_ms("stupid");
        assert_eq!(result, Err("ERROR: illegal"));
    }

    #[test]
    fn test_case_insensitive() {
        let result = check_ms("You are STUPID");
        assert_eq!(result, Err("ERROR: illegal"));
    }
}
