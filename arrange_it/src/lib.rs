pub fn arrange_phrase(phrase: &str) -> String {
    let mut words: Vec<(usize, String)> = Vec::new();

    for word in phrase.split_whitespace() {
        let mut number: usize = 0;
        let mut word_str = String::new();

        for c in word.chars() {
            if c.is_digit(10) {
                if let Some(n) = c.to_digit(10) {
                    number = n as usize;
                }
            } else {
                word_str.push(c);
            }
        }

        words.push((number, word_str));
    }

    words.sort_by_key(|&(num, _)| num);

    let result: String = words.into_iter().map(|(_, word)| word).collect::<Vec<String>>().join(" ");

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arrange_phrase() {
        let input = "is2 Thi1s T4est 3a";
        let expected = "This is a Test";
        assert_eq!(arrange_phrase(input), expected);

        let input2 = "Fo1r the2 g3ood 4of th5e pe6ople";
        let expected2 = "For the good of the people";
        assert_eq!(arrange_phrase(input2), expected2);
    }

    #[test]
    fn test_empty_string() {
        let input = "";
        let expected = "";
        assert_eq!(arrange_phrase(input), expected);
    }

    #[test]
    fn test_single_word() {
        let input = "hello1";
        let expected = "hello";
        assert_eq!(arrange_phrase(input), expected);
    }
}
