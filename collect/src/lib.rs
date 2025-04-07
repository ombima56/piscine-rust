pub fn bubble_sort(arr: &mut [i32]) {
    for i in 0..arr.len() {
        for j in 0..arr.len() - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble_sort() {
        let mut arr = [3, 2, 4, 5, 1, 7];
        bubble_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5, 7]);
    }
}
