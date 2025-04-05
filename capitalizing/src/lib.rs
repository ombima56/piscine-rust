pub fn capitalize_first(input: &str) -> String {
    if let Some(first_char) = input.chars().next() {
        let rest = &input[1..];
        first_char.to_uppercase().to_string() + rest
    } else {
        String::new()
    }
}

pub fn title_case(input: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = true;
    for c in input.chars() {
        if c.is_whitespace() {
            result.push(c);
            capitalize_next = true;
        } else {
            if capitalize_next {
                result.push(c.to_uppercase().next().unwrap_or(c));
                capitalize_next = false;
            } else {
                result.push(c);
            }
        }
    }
    result
}

pub fn change_case(input: &str) -> String {
    let mut result = String::new();
    for c in input.chars() {
        if c.is_lowercase() {
            result.push(c.to_uppercase().next().unwrap_or(c));
        } else if c.is_uppercase() {
            result.push(c.to_lowercase().next().unwrap_or(c));
        } else {
            result.push(c);
        }
    }
    result
}

