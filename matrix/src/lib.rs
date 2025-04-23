mod scalar;
use crate::scalar::Scalar;

#[derive(Debug, PartialEq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T: Scalar + Clone> Matrix<T> {
    pub fn new() -> Matrix<T> {
        Matrix(vec![vec![T::zero()]])
    }

    pub fn zero(row: usize, col: usize) -> Matrix<T> {
        Matrix(vec![vec![T::zero(); col]; row])
    }

    pub fn identity(n: usize) -> Matrix<T> {
        let mut matrix = vec![vec![T::zero(); n]; n];
        for i in 0..n {
            matrix[i][i] = T::one();
        }
        Matrix(matrix)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let m: Matrix<u32> = Matrix(vec![vec![0, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 0, 0]]);
        println!("{:?}", m);    
    }
    
    #[test]
    fn test_zero() {
        let m: Matrix<u32> = Matrix::zero(3, 4);
        println!("{:?}", m);
    }
}
