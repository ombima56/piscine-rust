use std::ops::Add;
use std::fmt::Debug;
use std::clone::Clone;
use std::cmp::PartialEq;

pub trait Scalar: Add<Output = Self> + std::ops::Mul<Output = Self> + Clone + Debug + PartialEq {
    fn zero() -> Self;
}

impl Scalar for i64 {
    fn zero() -> Self {
        0
    }
}

impl Scalar for i32 {
    fn zero() -> Self {
        0
    }
}

impl Scalar for f64 {
    fn zero() -> Self {
        0.0
    }
}

impl Scalar for f32 {
    fn zero() -> Self {
        0.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Vector<T: Scalar>(pub Vec<T>);

impl<T: Scalar> Add for Vector<T> {
    type Output = Option<Vector<T>>;

    fn add(self, other: Self) -> Self::Output {
        if self.0.len() != other.0.len() {
            return None;
        }
        let mut result = Vec::with_capacity(self.0.len());
        for (a, b) in self.0.iter().zip(other.0.iter()) {
            result.push(a.clone() + b.clone());
        }
        Some(Vector(result))
    }
}

impl<T: Scalar> Vector<T> {
    pub fn new(vec: Vec<T>) -> Self {
        Vector(vec)
    }

    pub fn dot(&self, other: &Self) -> Option<T> {
        if self.0.len() != other.0.len() {
            return None;
        }
        let mut sum = T::zero();
        for (a, b) in self.0.iter().zip(other.0.iter()) {
            sum = sum + a.clone() * b.clone();
        }
        Some(sum)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_add() {
        let vec1 = Vector::new(vec![1, 2, 3]);
        let vec2 = Vector::new(vec![4, 5, 6]);
        let result = vec1 + vec2;
        assert_eq!(result, Some(Vector::new(vec![5, 7, 9])));
    }
    #[test]
    fn test_vector_add_different_length() {
        let vec1 = Vector::new(vec![1, 2, 3]);
        let vec2 = Vector::new(vec![4, 5]);
        let result = vec1 + vec2;
        assert_eq!(result, None);
    }
    #[test]
    fn test_vector_dot() {
        let vec1 = Vector::new(vec![1, 2, 3]);
        let vec2 = Vector::new(vec![4, 5, 6]);
        let result = vec1.dot(&vec2);
        assert_eq!(result, Some(32));
    }
    #[test]
    fn test_vector_dot_different_length() {
        let vec1 = Vector::new(vec![1, 2, 3]);
        let vec2 = Vector::new(vec![4, 5]);
        let result = vec1.dot(&vec2);
        assert_eq!(result, None);
    }
    #[test]
    fn test_vector_dot_empty() {
        let vec1 = Vector::new(vec![]);
        let vec2 = Vector::new(vec![]);
        let result = vec1.dot(&vec2);
        assert_eq!(result, Some(0));
    }
}
