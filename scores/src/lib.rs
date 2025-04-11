pub fn score(s: &str) -> u64 {
    s.chars()
        .map(|c| match c.to_ascii_uppercase() {
            'A' | 'E' | 'I' | 'O' | 'U' | 'L' | 'N' | 'R' | 'S' | 'T' => 1,
            'D' | 'G' => 2,
            'B' | 'C' | 'M' | 'P' => 3,
            'F' | 'H' | 'V' | 'W' | 'Y' => 4,
            'K' => 5,
            'J' | 'X' => 8,
            'Q' | 'Z' => 10,
            _ => 0,
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score() {
        assert_eq!(score("a"), 1);
        assert_eq!(score("ã ê Á?"), 0);
        assert_eq!(score("ThiS is A Test"), 14);
        assert_eq!(score("ThiS is A Test!"), 14);
    }
}
