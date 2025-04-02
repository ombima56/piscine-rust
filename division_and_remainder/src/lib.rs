pub fn divide(x: i32, y: i32) -> (i32, i32) {
    let quotient = x / y;
    let remainder = x % y;
    (quotient, remainder)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divide() {
        assert_eq!(divide(10, 3), (3, 1));
        assert_eq!(divide(10, -3), (-3, 1));
        assert_eq!(divide(9, 4), (2, 1));
    }
}
