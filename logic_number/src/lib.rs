pub fn number_logic(num: u32) -> bool {
    let num_str = num.to_string();
    let power = num_str.len() as u32;

    let sum: u32 = num_str
        .chars()
        .map(|c| c.to_digit(10).unwrap().pow(power))
        .sum();

    sum == num
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn best_number_logic() {
        let array = [9, 10, 153, 154];
        for pat in &array {
            if number_logic(*pat) == true {
                assert_eq!(number_logic(*pat), true);
            }
            if number_logic(*pat) == false {
                assert_eq!(number_logic(*pat), false);
            }
        }
    }
}
