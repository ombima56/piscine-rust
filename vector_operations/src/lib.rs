use std::ops::{Add, Sub};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ThreeDVector<T> {
    pub i: T,
    pub j: T,
    pub k: T,
}

impl<T> Add for ThreeDVector<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            i: self.i + other.i,
            j: self.j + other.j,
            k: self.k + other.k,
        }
    }
}

impl<T> Sub for ThreeDVector<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            i: self.i - other.i,
            j: self.j - other.j,
            k: self.k - other.k,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let vec1 = ThreeDVector { i: 1, j: 2, k: 3 };
        let vec2 = ThreeDVector { i: 4, j: 5, k: 6 };
        let result = vec1 + vec2;
        assert_eq!(result, ThreeDVector { i: 5, j: 7, k: 9 });
    }
    #[test]
    fn test_sub() {
        let vec1 = ThreeDVector { i: 5, j: 7, k: 9 };
        let vec2 = ThreeDVector { i: 1, j: 2, k: 3 };
        let result = vec1 - vec2;
        assert_eq!(result, ThreeDVector { i: 4, j: 5, k: 6 });
    }
}
