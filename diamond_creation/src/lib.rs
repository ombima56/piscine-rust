pub fn get_diamond(c: char) -> Vec<String> {
    let mut result = Vec::new();

    let n = (c as u8 - b'A') as usize; // distance from 'A'
    let _width = n * 2 + 1;

    // Generate the top half
    for i in 0..=n {
        let letter = (b'A' + i as u8) as char;
        let outer_spaces = n - i;
        let inner_spaces = if i == 0 { 0 } else { i * 2 - 1 };

        let mut line = String::new();
        line.push_str(&" ".repeat(outer_spaces));
        line.push(letter);
        if inner_spaces > 0 {
            line.push_str(&" ".repeat(inner_spaces));
            line.push(letter);
        }
        line.push_str(&" ".repeat(outer_spaces));
        result.push(line);
    }

    // Generate the bottom half by reversing the top half (excluding middle row)
    for i in (0..n).rev() {
        result.push(result[i].clone());
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let diamond = get_diamond('C');
        assert_eq!(diamond.len(), 5);
        assert_eq!(diamond[0], "  A  ");
        assert_eq!(diamond[1], " B B ");
        assert_eq!(diamond[2], "C   C");
        assert_eq!(diamond[3], " B B ");
        assert_eq!(diamond[4], "  A  ");

        let diamond = get_diamond('E');
        assert_eq!(diamond.len(), 9);
        assert_eq!(diamond[0], "    A    ");
        assert_eq!(diamond[1], "   B B   ");
        assert_eq!(diamond[2], "  C   C  ");
        assert_eq!(diamond[3], " D     D ");
        assert_eq!(diamond[4], "E       E");
        assert_eq!(diamond[5], " D     D ");
        assert_eq!(diamond[6], "  C   C  ");
        assert_eq!(diamond[7], "   B B   ");
        assert_eq!(diamond[8], "    A    ");
    }
}
