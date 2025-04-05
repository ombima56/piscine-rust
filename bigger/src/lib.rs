// Create a function named bigger that gets the biggest positive number in the HashMap.
pub fn bigger(h: HashMap<&str, i32>) -> i32 {
    *h.values().max().unwrap_or(&i32::MIN) // Get the maximum value, return MIN if HashMap is empty
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_bigger() {
        let hash = HashMap::from_iter([
            ("Daniel", 122),
            ("Ashley", 333),
            ("Katie", 334),
            ("Robert", 14),
        ]);
        
        assert_eq!(bigger(hash), 334);
    }

    #[test]
    fn test_bigger_empty() {
        let hash: HashMap<&str, i32> = HashMap::new();
        assert_eq!(bigger(hash), i32::MIN);
    }

    #[test]
    fn test_bigger_single_value() {
        let hash = HashMap::from([("Alice", 50)]);
         assert_eq!(bigger(hash), 50);
    }
}
