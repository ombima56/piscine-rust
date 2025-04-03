pub fn tic_tac_toe(table: [[char; 3]; 3]) -> String {
    if horizontal('X', table) || vertical('X', table) || diagonals('X', table) {
        return "player X won".to_string();
    }
    if horizontal('O', table) || vertical('O', table) || diagonals('O', table) {
        return "player O won".to_string();
    }
    "tie".to_string()
}

pub fn diagonals(player: char, table: [[char; 3]; 3]) -> bool {
    // Check the first diagonal (top-left to bottom-right)
    if table[0][0] == player && table[1][1] == player && table[2][2] == player {
        return true;
    }
    // Check the second diagonal (top-right to bottom-left)
    if table[0][2] == player && table[1][1] == player && table[2][0] == player {
        return true;
    }
    false
}

pub fn horizontal(player: char, table: [[char; 3]; 3]) -> bool {
    for row in table.iter() {
        if row[0] == player && row[1] == player && row[2] == player {
            return true;
        }
    }
    false
}

pub fn vertical(player: char, table: [[char; 3]; 3]) -> bool {
    for col in 0..3 {
        if table[0][col] == player && table[1][col] == player && table[2][col] == player {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tic_tac_toe() {
        assert_eq!(tic_tac_toe([['O', 'X', 'O'], ['O', 'P', 'X'], ['X', '#', 'X']]), "tie");
        assert_eq!(tic_tac_toe([['X', 'O', 'O'], ['X', 'O', 'O'], ['#', 'O', 'X']]), "player O won");
        assert_eq!(tic_tac_toe([['O', 'O', 'X'], ['O', 'X', 'O'], ['X', '#', 'X']]), "player X won");
    }

    #[test]
    fn test_diagonals() {
        assert_eq!(diagonals('X', [['O', 'O', 'X'], ['O', 'X', 'O'], ['X', '#', 'X']]), true);
        assert_eq!(diagonals('O', [['O', 'O', 'X'], ['O', 'X', 'O'], ['X', '#', 'X']]), false);
    }

    #[test]
    fn test_horizontal() {
        assert_eq!(horizontal('X', [['O', 'O', 'X'], ['X', 'X', 'X'], ['#', 'O', 'X']]), true);
        assert_eq!(horizontal('O', [['O', 'O', 'X'], ['X', 'X', 'X'], ['#', 'O', 'X']]), false);
    }

    #[test]
    fn test_vertical() {
        assert_eq!(vertical('X', [['O', 'O', 'X'], ['X', 'X', 'X'], ['#', 'O', 'X']]), true);
        assert_eq!(vertical('O', [['O', 'O', 'X'], ['X', 'X', 'X'], ['#', 'O', 'X']]), false);
    }
}
