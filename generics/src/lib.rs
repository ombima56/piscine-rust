pub fn identity<T>(v: T) -> T {
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity_with_string() {
        let input = "Hello, world!";
        let expected = "Hello, world!";
        let result = identity(input);
        assert_eq!(result, expected);
    }
    #[test]
    fn test_identity_with_integer() {
        let input = 3;
        let expected = 3;
        let result = identity(input);
        assert_eq!(result, expected);
    }
    #[test]
    fn test_identity_with_float() {
        let input = 3.14;
        let expected = 3.14;
        let result = identity(input);
        assert_eq!(result, expected);
    }
}
