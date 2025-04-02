pub fn str_len(s: &str) -> usize {
    s.chars().count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_str_len() {
        assert_eq!(str_len("hello"), 5);
        assert_eq!(str_len(""), 0);
        assert_eq!(str_len("camelCase"), 9);
        assert_eq!(str_len("olá!"), 4);
    }
}
