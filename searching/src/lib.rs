pub fn search(array: &[i32], key: i32) -> Option<usize> {
    array.binary_search(&key).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search() {
        let array = [1, 3, 4, 6, 8, 9, 11];
        assert_eq!(search(&array, 6), Some(3));
        assert_eq!(search(&array, 1), Some(0));
        assert_eq!(search(&array, 11), Some(6));
        assert_eq!(search(&array, 5), None);
    }
}
