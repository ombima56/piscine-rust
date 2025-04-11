pub fn num_to_ordinal(x: u32) -> String {
    let suffix = match x % 10 {
        1 if x % 100 != 11 => "st",
        2 if x % 100 != 12 => "nd",
        3 if x % 100 != 13 => "rd",
        _ => "th",
    };

    format!("{}{}", x, suffix)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_to_ordinal() {
        assert_eq!(num_to_ordinal(1), "1st");
        assert_eq!(num_to_ordinal(2), "2nd");
        assert_eq!(num_to_ordinal(3), "3rd");
        assert_eq!(num_to_ordinal(4), "4th");
        assert_eq!(num_to_ordinal(11), "11th");
        assert_eq!(num_to_ordinal(12), "12th");
        assert_eq!(num_to_ordinal(13), "13th");
        assert_eq!(num_to_ordinal(21), "21st");
        assert_eq!(num_to_ordinal(22), "22nd");
        assert_eq!(num_to_ordinal(23), "23rd");
    }
}

