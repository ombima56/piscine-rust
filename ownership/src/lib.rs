pub fn first_subword(mut s: String) -> String {
    s = s.trim().to_string();

    if s.is_empty() {
        return String::new();
    }

    let mut first_subword = String::new();
    let mut chars = s.chars().peekable();

    if let Some(first_char) = chars.next() {
        first_subword.push(first_char);

        for c in chars {
            if c == '_' || (c.is_uppercase() && !first_subword.is_empty()) {
                break;
            }
            first_subword.push(c);
        }
    }

    first_subword
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_subword() {
        assert_eq!(first_subword("helloWorld".to_string()), "hello");
        assert_eq!(first_subword("snake_case".to_string()), "snake");
        assert_eq!(first_subword("CamelCase".to_string()), "Camel");
        assert_eq!(first_subword("just".to_string()), "just");
        assert_eq!(first_subword("OneWord".to_string()), "One");
        assert_eq!(first_subword("multiple_words_here".to_string()), "multiple");
    }
}

