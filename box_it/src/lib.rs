pub fn transform_and_save_on_heap(s: String) -> Box<Vec<u32>> {
    let vec: Vec<u32> = s
        .split_whitespace()
        .map(|num| {
            if let Some(stripped) = num.strip_suffix('k') {
                (stripped.parse::<f32>().unwrap_or(0.0) * 1000.0).round() as u32
            } else {
                num.parse::<u32>().unwrap_or(0)
            }
        })
        .collect();

    Box::new(vec)
}

pub fn take_value_ownership(a: Box<Vec<u32>>) -> Vec<u32> {
    *a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform_and_save_on_heap_with_k_and_normal_values() {
        let input = String::from("5.5k 8.9k 32");
        let expected = vec![5500, 8900, 32];
        let result = transform_and_save_on_heap(input);
        assert_eq!(*result, expected);
    }

    #[test]
    fn test_transform_and_save_on_heap_with_only_numbers() {
        let input = String::from("100 200 300");
        let expected = vec![100, 200, 300];
        let result = transform_and_save_on_heap(input);
        assert_eq!(*result, expected);
    }

    #[test]
    fn test_take_value_ownership_unboxes_correctly() {
        let input = Box::new(vec![1000, 2000, 3000]);
        let expected = vec![1000, 2000, 3000];
        let result = take_value_ownership(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_transform_and_save_on_heap_with_invalid_numbers() {
        let input = String::from("abc 5k -3 4.2k");
        let expected = vec![0, 5000, 0, 4200];
        let result = transform_and_save_on_heap(input);
        assert_eq!(*result, expected);
    }
}

