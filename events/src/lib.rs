use chrono::Duration;
use std::fmt;

#[derive(Debug, Eq, PartialEq)]
pub enum Position {
    Top,
    Bottom,
    Center,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Notification {
    pub size: u32,
    pub color: (u8, u8, u8),
    pub position: Position,
    pub content: String,
}

#[derive(Debug)]
pub enum Event<'a> {
    Remainder(&'a str),
    Registration(Duration),
    Appointment(&'a str),
    Holiday,
}

use Event::*;

impl Event<'_> {
    pub fn notify(&self) -> Notification {
        match self {
            Remainder(text) => Notification {
                size: 50,
                color: (50, 50, 50),
                position: Position::Bottom,
                content: text.to_string(),
            },
            Registration(duration) => {
                let total_seconds = duration.num_seconds();
                let hours = total_seconds / 3600;
                let minutes = (total_seconds % 3600) / 60;
                let seconds = total_seconds % 60;

                Notification {
                    size: 30,
                    color: (255, 2, 22),
                    position: Position::Top,
                    content: format!(
                        "You have {}H:{}M:{}S left before the registration ends",
                        hours, minutes, seconds
                    ),
                }
            }
            Appointment(text) => Notification {
                size: 100,
                color: (200, 200, 3),
                position: Position::Center,
                content: text.to_string(),
            },
            Holiday => Notification {
                size: 25,
                color: (0, 255, 0),
                position: Position::Top,
                content: "Enjoy your holiday".to_string(),
            },
        }
    }
}

impl fmt::Display for Notification {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (r, g, b) = self.color;
        
        // Manually construct the ANSI-colored string format
        let colored_content = format!("[38;2;{};{};{}m{}[0m", r, g, b, self.content);
        
        write!(f, "({:?}, {}, {})", self.position, self.size, colored_content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
