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
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
