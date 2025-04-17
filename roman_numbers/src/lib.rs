use crate::RomanDigit::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RomanDigit {
    Nulla,
    I,
    V,
    X,
    L,
    C,
    D,
    M,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RomanNumber(pub Vec<RomanDigit>);

impl From<u32> for RomanDigit {
    fn from(value: u32) -> Self {
        match value {
            0 => Nulla,
            1 => I,
            5 => V,
            10 => X,
            50 => L,
            100 => C,
            500 => D,
            1000 => M,
            _ => panic!("Invalid digit for RomanDigit"),
        }
    }
}

impl From<u32> for RomanNumber {
    fn from(mut num: u32) -> Self {
        if num == 0 {
            return RomanNumber(vec![Nulla]);
        }

        let mut result = Vec::new();

        let table = [
            (1000, M),
            (900, C), (900, M), // CM
            (500, D),
            (400, C), (400, D), // CD
            (100, C),
            (90, X), (90, C),   // XC
            (50, L),
            (40, X), (40, L),   // XL
            (10, X),
            (9, I), (9, X),     // IX
            (5, V),
            (4, I), (4, V),     // IV
            (1, I),
        ];

        let mut i = 0;
        while i < table.len() {
            let (value, digit) = table[i];
            if num >= value {
                // Check for subtractive pair
                if matches!(value, 900 | 400 | 90 | 40 | 9 | 4) {
                    result.push(digit);
                    result.push(table[i + 1].1);
                    num -= value;
                    i += 2;
                } else {
                    result.push(digit);
                    num -= value;
                    i += 1;
                }
            } else {
                i += 1;
            }
        }

        RomanNumber(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roman_digit_from_u32() {
        assert_eq!(RomanDigit::from(1), I);
        assert_eq!(RomanDigit::from(5), V);
        assert_eq!(RomanDigit::from(10), X);
        assert_eq!(RomanDigit::from(50), L);
        assert_eq!(RomanDigit::from(100), C);
        assert_eq!(RomanDigit::from(500), D);
        assert_eq!(RomanDigit::from(1000), M);
    }
}
