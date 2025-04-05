use std::collections::HashMap;

pub fn bigger(h: HashMap<&str, i32>) -> i32 {
    let mut max_value = i32::MIN;
    for (_, value) in h.iter() {
        if *value > max_value {
            max_value = *value;
        }
    }
    max_value
}
