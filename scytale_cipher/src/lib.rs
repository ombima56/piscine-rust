pub fn scytale_cipher(message: String, i: u32) -> String {
    let columns = i as usize;
    if columns == 0 || columns == 1 {
        return message;
    }
    
    let message_chars: Vec<char> = message.chars().collect();
    let message_len = message_chars.len();
    let rows = (message_len + columns - 1) / columns;
    
    // Create a grid and fill it with placeholders
    let mut grid = vec![vec![' '; columns]; rows];
    
    // Fill grid row by row with message characters
    let mut char_idx = 0;
    for row in 0..rows {
        for col in 0..columns {
            if char_idx < message_len {
                grid[row][col] = message_chars[char_idx];
                char_idx += 1;
            }
        }
    }
    
    // Read grid column by column to produce cipher text
    let mut result = String::with_capacity(message_len);
    for col in 0..columns {
        for row in 0..rows {
            let char = grid[row][col];
            result.push(char);
        }
    }
    
    result.trim_end().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn it_works() {
        assert_eq!(scytale_cipher(String::from("scytale Code"), 6), "sec yCtoadle");
        assert_eq!(scytale_cipher(String::from("scytale Code"), 8), "sCcoydtea l e");
    }
}