pub fn arrange_phrase(phrase: &str) -> String {
    let mut words: Vec<&str> = phrase.split_whitespace().collect();
    
    words.sort_by_key(|word| {
        word.chars()
            .find(|c| c.is_digit(10))
            .map(|c| c.to_digit(10).unwrap())
            .unwrap_or(0)
    });
    
    words.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arrange_phrase() {
        let input = "is2 Thi1s T4est 3a";
        let expected = "Thi1s is2 3a T4est";
        assert_eq!(arrange_phrase(input), expected);
    }
}
