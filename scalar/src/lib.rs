pub fn sum(a: u8, b: u8) -> u8 {
    a + b
    
}

pub fn diff(a: i16, b: i16) -> i16 {
    a - b
}

pub fn pro(a: i8, b: i8) -> i8 {
    a * b
}

pub fn quo(a: f32, b: f32) -> f32 {
    a / b

}

pub fn rem(a: f32, b: f32) -> f32 {
   a % b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        assert_eq!(sum(100, 30), 130);
        assert_eq!(sum(30, 150), 180);
    }

    #[test]
    fn test_diff() {
        assert_eq!(diff(234, 2), 232);
        assert_eq!(diff(10, 20), -10);
    }

    #[test]
    fn test_pro() {
        assert_eq!(pro(10, 10), 100);
        assert_eq!(pro(11, 2), 22);
    }

    #[test]
    fn test_quo() {
        assert_eq!(quo(100.0, 2.0), 50.0);
    }

    #[test]
    fn test_rem() {
        assert_eq!(rem(10.0, 3.0), 1.0);
    }
}
