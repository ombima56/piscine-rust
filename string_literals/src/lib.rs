pub fn is_empty(v: &str) -> bool {
    v.is_empty()
}

pub fn is_ascii(v: &str) -> bool {
    v.chars().all(|c| c.is_ascii())
}

pub fn contains(v: &str, pat: &str) -> bool {
    v.contains(pat)
}

pub fn split_at(v: &str, index: usize) -> (&str, &str) {
    assert!(index <= v.len(), "Index out of bounds");
    v.split_at(index)
}

pub fn find(v: &str, pat: char) -> usize {
    v.find(pat).unwrap_or(usize::MAX)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_empty() {
        assert!(is_empty(""));
        assert!(!is_empty("rust"));
    }

    #[test]
    fn test_is_ascii() {
        assert!(is_ascii("rust"));
        assert!(!is_ascii("你好"));
    }

    #[test]
    fn test_contains() {
        assert!(contains("rust", "ru"));
        assert!(!contains("rust", "go"));
    }

    #[test]
    fn test_split_at() {
        assert_eq!(split_at("rust", 2), ("ru", "st"));
        assert_eq!(split_at("hello", 3), ("hel", "lo"));
    }

    #[test]
    fn test_find() {
        assert_eq!(find("rust", 'u'), 1);
        assert_eq!(find("hello", 'o'), 4); 
        assert_eq!(find("test", 'z'), usize::MAX);
    }
}

