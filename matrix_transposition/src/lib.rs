#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Matrix(pub (i32, i32), pub (i32, i32));

pub fn transpose(m: Matrix) -> Matrix {
    let Matrix((a, b), (c, d)) = m;
    
    Matrix((a, c), (b, d))
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_transpose() {
        assert_eq!(transpose(Matrix((1, 2), (3, 4))), Matrix((1, 3), (2, 4)));
        assert_eq!(transpose(Matrix((5, 6), (7, 8))), Matrix((5, 7), (6, 8)));
        
        let matrix = Matrix((1, 2), (3, 4));
        assert_eq!(transpose(transpose(matrix.clone())), matrix);
    }
}
