use std::f64;

pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    (c, (c as f64).exp(), (c as f64).ln())
}

pub fn str_function(a: String) -> (String, String) {
    let exp_values = a
        .split_whitespace()
        .map(|s| s.parse::<f64>().unwrap().exp().to_string())
        .collect::<Vec<String>>()
        .join(" ");

    (a, exp_values)
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let log_values = b
        .iter()
        .map(|&x| (x.abs() as f64).ln()) // Compute ln() of absolute values
        .collect();

    (b, log_values)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nbr_function() {
        let result = nbr_function(0);
        assert_eq!(result, (0, 1.0, f64::NEG_INFINITY));

        let result = nbr_function(5);
        assert_eq!(result, (5, 148.4131591025766, 1.6094379124341003));
    }

    #[test]
    fn test_vec_function() {
        let result = vec_function(vec![1, 2, 4, 5]);
        assert_eq!(
            result,
            (
                vec![1, 2, 4, 5],
                vec![0.0, 0.6931471805599453, 1.3862943611198906, 1.6094379124341003]
            )
        );
    }

    #[test]
    fn test_str_function() {
        let result = str_function("1 2 4 5 6".to_owned());
        assert_eq!(
            result,
            (
                "1 2 4 5 6".to_owned(),
                "2.718281828459045 7.38905609893065 54.598150033144236 148.4131591025766 403.4287934927351".to_owned()
            )
        );
    }
}
