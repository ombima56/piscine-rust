pub fn is_permutation(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }

    let mut char_counts = HashMap::new();

    for char in s1.chars() {
        let count = char_counts.entry(char).or_insert(0);
        *count += 1;
    }

    for char in s2.chars() {
        let count = char_counts.entry(char).or_insert(0);
        *count -= 1;
    }

    for (_, count) in char_counts.iter() {
        if *count != 0 {
            return false;
        }
    }
    true
}
