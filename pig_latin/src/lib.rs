pub fn pig_latin(text: &str) -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u'];

    if text.is_empty() {
        return String::new();
    }

    let lower = text.to_lowercase();
    let chars: Vec<char> = lower.chars().collect();

    if vowels.contains(&chars[0]) {
        return format!("{}ay", lower);
    }

    let mut idx = 0;

    // Special case for consonant followed by "qu"
    if chars.len() >= 3 && !vowels.contains(&chars[0]) && chars[1] == 'q' && chars[2] == 'u' {
        idx = 3;
    } else {
        while idx < chars.len() && !vowels.contains(&chars[idx]) {
            idx += 1;
        }
    }

    let (start, end) = chars.split_at(idx);
    let result = format!("{}{}ay", end.iter().collect::<String>(), start.iter().collect::<String>());
    result
}


