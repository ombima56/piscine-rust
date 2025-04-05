use std::collections::HashMap;

pub fn mean(list: &[i32]) -> f64 {
    if list.is_empty() {
        return 0.0;
    }
    let sum: i32 = list.iter().sum();
    sum as f64 / list.len() as f64
}

pub fn median(list: &[i32]) -> i32 {
    let mut sorted_list = list.to_vec();
    sorted_list.sort();

    let mid = sorted_list.len() / 2;
    if sorted_list.len() % 2 == 0 {
        let left = sorted_list[mid - 1];
        let right = sorted_list[mid];
        (left + right) / 2
    } else {
        sorted_list[mid]
    }
}

pub fn mode(list: &[i32]) -> i32 {
    let mut counts = HashMap::new();

    for &value in list {
        let count = counts.entry(value).or_insert(0);
        *count += 1;
    }

    let mut mode_value = 0;
    let mut max_count = 0;

    for (value, count) in counts.iter() {
        if *count > max_count {
            max_count = *count;
            mode_value = *value;
        }
    }

    mode_value
}
