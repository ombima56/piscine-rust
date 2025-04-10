use std::cmp;

fn edit_distance(a: &str, b: &str) -> usize {
    let a_len = a.len();
    let b_len = b.len();
    let mut dp = vec![vec![0; b_len + 1]; a_len + 1];

    for i in 0..=a_len {
        dp[i][0] = i;
    }

    for j in 0..=b_len {
        dp[0][j] = j;
    }

    for i in 1..=a_len {
        for j in 1..=b_len {
            dp[i][j] = cmp::min(
                cmp::min(
                    dp[i - 1][j] + 1,
                    dp[i][j - 1] + 1,
                ),
                dp[i - 1][j - 1] + if a.as_bytes()[i - 1] == b.as_bytes()[j - 1] { 0 } else { 1 },
            );
        }
    }

    dp[a_len][b_len]
}

fn is_camel_case(s: &str) -> bool {
    // A string is camelCase if it starts with a lowercase letter and contains no underscores
    s.chars().next().map_or(false, |c| c.is_lowercase()) && !s.contains('_')
}

fn is_snake_case(s: &str) -> bool {
    // A string is snake_case if it contains only lowercase letters and underscores
    s.chars().all(|c| c.is_lowercase() || c == '_') && s.contains('_')
}

pub fn expected_variable(compared: &str, expected: &str) -> Option<String> {
    // Convert the strings to lowercase for case insensitive comparison
    let compared_lower = compared.to_lowercase();
    let expected_lower = expected.to_lowercase();

    // Check if compared and expected strings are either camel case or snake case
    if !(is_camel_case(compared) || is_snake_case(compared)) {
        return None;
    }

    // Calculate edit distance
    let distance = edit_distance(&compared_lower, &expected_lower);
    let max_len = cmp::max(compared.len(), expected.len());

    // Calculate similarity percentage
    let similarity = (1.0 - (distance as f64 / max_len as f64)) * 100.0;

    // If the similarity is more than 50%, return the value as a string with '%' symbol
    if similarity > 50.0 {
        return Some(format!("{:.0}%", similarity));
    }

    None
}
