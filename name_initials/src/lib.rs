pub fn initials(names: Vec<&str>) -> Vec<String> {
    names
        .iter()
        .map(|name| {
            name.split_whitespace()
                .filter_map(|word| word.chars().next())
                .map(|c| format!("{}.", c.to_ascii_uppercase()))
                .collect::<Vec<_>>()
                .join(" ")
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initials() {
        let names = vec!["Harry Potter", "Someone Else", "J. L.", "Barack Obama"];
        let result = initials(names);
        let expected = vec!["H. P.", "S. E.", "J. L.", "B. O."];

        assert_eq!(result, expected);
    }
}
