pub fn stars(n: u32) -> String {
    let mut result = String::new();
    for _ in 0..n {
        result.push('*');
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stars() {
        assert_eq!(stars(1), "*");
        assert_eq!(stars(4), "****");
        assert_eq!(stars(5), "*****");
    }
}
