pub fn stars(n: u32) -> String {
    let mut result = String::new();
    // Calculates 2^n (2 raised to the power of n), and stores it in num_stars.
    let num_stars = 2u32.pow(n);
    for _ in 0..num_stars {
        result.push('*');
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stars() {
        assert_eq!(stars(1), "**");
        assert_eq!(stars(4), "****************");
        assert_eq!(stars(5), "********************************");
    }
}
