pub fn edit_distance(source: &str, target: &str) -> usize {
    let m = source.len();
    let n = target.len();

    // Create a matrix with dimensions (m + 1) x (n + 1)
    let mut dp = vec![vec![0; n + 1]; m + 1];

    // Initialize base cases
    for i in 0..=m {
        dp[i][0] = i; // Deleting all characters from source
    }
    for j in 0..=n {
        dp[0][j] = j; // Inserting all characters into source
    }

    // Fill the dp table
    for i in 1..=m {
        for j in 1..=n {
            if source.chars().nth(i - 1) == target.chars().nth(j - 1) {
                dp[i][j] = dp[i - 1][j - 1]; // No change needed
            } else {
                dp[i][j] = *[
                    dp[i - 1][j] + 1, // Deletion
                    dp[i][j - 1] + 1, // Insertion
                    dp[i - 1][j - 1] + 1, // Substitution
                ]
                .iter()
                .min()
                .unwrap();
            }
        }
    }

    dp[m][n] // Return the value in the bottom-right corner of the matrix
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
