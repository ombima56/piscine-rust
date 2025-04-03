pub fn delete_and_backspace(s: &mut String) {
    let mut stack = Vec::new();
    let mut delete_count = 0;

    for c in s.chars() {
        match c {
            '-' => {
                stack.pop();
            },
            '+' => {
                delete_count += 1;
            },
            _ => {
                if delete_count > 0 {
                    delete_count -= 1;
                } else {
                    stack.push(c);
                }
            }
        }
    }

    *s = stack.into_iter().collect();
}

pub fn do_operations(v: &mut [String]) {
    for expr in v.iter_mut() {
        let mut parts = expr.split(|c| c == '+' || c == '-');
        if let (Some(lhs), Some(rhs)) = (parts.next(), parts.next()) {
            if let (Ok(left), Ok(right)) = (lhs.parse::<i32>(), rhs.parse::<i32>()) {
                let result = if expr.contains('+') {
                    left + right
                } else {
                    left - right
                };
                *expr = result.to_string();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delete_and_backspace() {
        let mut input = "bpp--o+er+++sskroi-++lcw".to_owned();
        delete_and_backspace(&mut input);
        assert_eq!(input, "borrow");
    }

    #[test]
    fn test_do_operations() {
        let mut input = [
            "2+2".to_owned(),
            "3+2".to_owned(),
            "10-3".to_owned(),
            "5+5".to_owned(),
        ];
        do_operations(&mut input);
        assert_eq!(input, ["4", "5", "7", "10"]);
    }
}
