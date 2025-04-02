pub fn factorial(num: u64) -> u64 {
    if num == 0 {
        1
    } else {
        num * factorial(num - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(5), 120);
        assert_eq!(factorial(10), 3628800);
        assert_eq!(factorial(19), 121645100408832000);
    }
}
