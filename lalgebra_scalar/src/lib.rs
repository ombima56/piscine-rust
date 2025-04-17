use std::ops::{Add, Sub, Mul, Div};

/// A trait for scalar types in linear algebra.
pub trait Scalar:
    Copy
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
{
    fn zero() -> Self;
    fn one() -> Self;
}

// IMplement Scalar for integer and float types
impl Scalar for u32 {
    fn zero() -> Self {
        0
    }
    fn one() -> Self {
        1
    }
}

impl Scalar for u64 {
    fn zero() -> Self {
        0
    }
    fn one() -> Self {
        1
    }
}

impl Scalar for i32 {
    fn zero() -> Self {
        0
    }
    fn one() -> Self {
        1
    }
}

impl Scalar for i64 {
    fn zero() -> Self {
        0
    }
    fn one() -> Self {
        1
    }
}

impl Scalar for f32 {
    fn zero() -> Self {
        0.0
    }
    fn one() -> Self {
        1.0
    }
}

impl Scalar for f64 {
    fn zero() -> Self {
        0.0
    }
    fn one() -> Self {
        1.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scalar_zero() {
        assert_eq!(u32::zero(), 0);
        assert_eq!(u64::zero(), 0);
        assert_eq!(i32::zero(), 0);
        assert_eq!(i64::zero(), 0);
        assert_eq!(f32::zero(), 0.0);
        assert_eq!(f64::zero(), 0.0);
    }
    #[test]
    fn test_scalar_one() {
        assert_eq!(u32::one(), 1);
        assert_eq!(u64::one(), 1);
        assert_eq!(i32::one(), 1);
        assert_eq!(i64::one(), 1);
        assert_eq!(f32::one(), 1.0);
        assert_eq!(f64::one(), 1.0);
    }
}
