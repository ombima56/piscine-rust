use case::CaseExt;

pub fn expected_variable(compared: &str, expected: &str) -> Option<String> {
    // Check if the compared string is in camel case or snake case
    // For camel case: either it's lowercase camel case or Pascal case (starts with uppercase)
    let is_camel = compared.is_camel_lowercase() ||
                  (compared.chars().next().map_or(false, |c| c.is_uppercase()) &&
                   compared.chars().skip(1).any(|c| c.is_uppercase()) &&
                   !compared.contains('_'));

    // For snake case: contains underscores
    let is_snake = compared.contains('_');

    // Special case: if it's a simple lowercase word with no underscores or uppercase letters,
    // it's neither camel case nor snake case
    let is_simple_word = !compared.contains('_') &&
                        !compared.chars().any(|c| c.is_uppercase());

    if (!is_camel && !is_snake) || is_simple_word {
        return None;
    }

    // Calculate edit distance
    let distance = edit_distance(compared, expected);

    // Calculate similarity percentage
    let expected_len = expected.len() as f64;
    let similarity = ((expected_len - distance as f64) / expected_len) * 100.0;

    // Return percentage as string if more than 50% alike
    if similarity > 50.0 {
        Some(format!("{}%", similarity.round() as i32))
    } else {
        None
    }
}

// Implementation of the edit_distance function
pub fn edit_distance(a: &str, b: &str) -> usize {
    // We need to normalize the strings to lowercase for case-insensitive comparison
    let a_lower = a.to_lowercase();
    let b_lower = b.to_lowercase();

    let a_chars: Vec<char> = a_lower.chars().collect();
    let b_chars: Vec<char> = b_lower.chars().collect();

    let len_a = a_chars.len();
    let len_b = b_chars.len();

    // Handle edge cases
    if len_a == 0 {
        return len_b;
    }
    if len_b == 0 {
        return len_a;
    }

    // Create a matrix to store the edit distances
    let mut matrix = vec![vec![0; len_b + 1]; len_a + 1];

    // Initialize the first row and column
    for i in 0..=len_a {
        matrix[i][0] = i;
    }
    for j in 0..=len_b {
        matrix[0][j] = j;
    }

    // Fill the matrix using dynamic programming
    for i in 1..=len_a {
        for j in 1..=len_b {
            let cost = if a_chars[i - 1] == b_chars[j - 1] { 0 } else { 1 };

            matrix[i][j] = (matrix[i - 1][j] + 1)
                .min(matrix[i][j - 1] + 1)
                .min(matrix[i - 1][j - 1] + cost);
        }
    }

    // Return the bottom-right cell which contains the edit distance
    matrix[len_a][len_b]
}