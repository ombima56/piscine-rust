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
            (1000, vec![M]),
            (900, vec![C, M]),  // CM
            (500, vec![D]),
            (400, vec![C, D]),  // CD
            (100, vec![C]),
            (90, vec![X, C]),   // XC
            (50, vec![L]),
            (40, vec![X, L]),   // XL
            (10, vec![X]),
            (9, vec![I, X]),    // IX
            (5, vec![V]),
            (4, vec![I, V]),    // IV
            (1, vec![I]),
        ];
        
        while num > 0 {
            for &(value, ref digits) in &table {
                if num >= value {
                    result.extend_from_slice(digits);
                    num -= value;
                    break;
                }
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
