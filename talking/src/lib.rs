pub fn talking(text: &str) -> &str {
    let trimmed = text.trim();

    if trimmed.is_empty() {
        "Just say something!"
    } else if trimmed.ends_with('?') {
        if trimmed.chars().filter(|c| c.is_ascii_alphabetic()).all(|c| c.is_ascii_uppercase()) {
            "Quiet, I am thinking!"
        } else {
            "Sure."
        }
    } else if trimmed.chars().filter(|c| c.is_ascii_alphabetic()).all(|c| c.is_ascii_uppercase()) {
        "There is no need to yell, calm down!"
    } else {
        "Interesting"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_yelling() {
        assert_eq!(talking("LEAVE ME ALONE!"), "There is no need to yell, calm down!");
        assert_eq!(talking("STOP!"), "There is no need to yell, calm down!");
    }

    #[test]
    fn test_question() {
        assert_eq!(talking("Is everything ok?"), "Sure.");
        assert_eq!(talking("Do you like Rust?"), "Sure.");
    }

    #[test]
    fn test_yelling_question() {
        assert_eq!(talking("HOW ARE YOU?"), "Quiet, I am thinking!");
        assert_eq!(talking("WHAT'S UP?"), "Quiet, I am thinking!");
    }

    #[test]
    fn test_empty_input() {
        assert_eq!(talking(""), "Just say something!");
        assert_eq!(talking("   "), "Just say something!");
    }

    #[test]
    fn test_other_input() {
        assert_eq!(talking("Rust is fun"), "Interesting");
        assert_eq!(talking("okay"), "Interesting");
    }
}


