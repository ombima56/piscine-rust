use std::collections::HashMap;

pub fn word_frequency_counter<'a>(words: &'a [&'a str]) -> HashMap<&'a str, usize> {
    let mut frequency_map = HashMap::new();
    for &word in words {
        *frequency_map.entry(word).or_insert(0) += 1;
    }
    frequency_map
}

pub fn nb_distinct_words(frequency_count: &HashMap<&str, usize>) -> usize {
    frequency_count.len()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_frequency_counter() {
        let words = ["apple", "banana", "apple", "orange", "banana", "banana"];
        let frequency_count = word_frequency_counter(&words);
        
        assert_eq!(frequency_count["apple"], 2);
        assert_eq!(frequency_count["banana"], 3);
        assert_eq!(frequency_count["orange"], 1);
    }

    #[test]
    fn test_nb_distinct_words() {
        let words = ["apple", "banana", "apple", "orange", "banana", "banana"];
        let frequency_count = word_frequency_counter(&words);
        
        assert_eq!(nb_distinct_words(&frequency_count), 3);
    }

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
