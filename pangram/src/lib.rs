pub fn is_pangram(s: &str) -> bool {
    let alphabet = "abcdefghijklmnopqrstuvwxyz";
    let s = s.to_lowercase();
    for c in alphabet.chars() {
        if !s.contains(c) {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pangram_true() {
        let input = "The quick brown fox jumps over the lazy dog";
        assert!(is_pangram(input));
    }

    #[test]
    fn test_pangram_false() {
        let input = "hello world";
        assert!(!is_pangram(input));
    }

    #[test]
    fn test_empty_string() {
        let input = "";
        assert!(!is_pangram(input));
    }

    #[test]
    fn test_pangram_with_punctuation() {
        let input = "Sphinx of black quartz, judge my vow!";
        assert!(is_pangram(input));
    }

    #[test]
    fn test_pangram_with_numbers_and_symbols() {
        let input = "1234567890 The quick brown fox jumps over the lazy dog!";
        assert!(is_pangram(input));
    }

    #[test]
    fn test_case_insensitivity() {
        let input = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        assert!(is_pangram(input));
    }
}
