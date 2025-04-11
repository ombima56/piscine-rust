pub fn spell(n: u64) -> String {
    let under_20 = [
        "", "one", "two", "three", "four", "five", "six", "seven",
        "eight", "nine", "ten", "eleven", "twelve", "thirteen",
        "fourteen", "fifteen", "sixteen", "seventeen", "eighteen", "nineteen",
    ];

    let tens = [
        "", "", "twenty", "thirty", "forty", "fifty",
        "sixty", "seventy", "eighty", "ninety",
    ];

    if n == 0 {
        return "zero".to_string();
    }

    if n < 20 {
        return under_20[n as usize].to_string();
    }

    if n < 100 {
        if n % 10 == 0 {
            return tens[(n / 10) as usize].to_string();
        } else {
            return format!("{}-{}", tens[(n / 10) as usize], spell(n % 10));
        }
    }

    if n < 1_000 {
        if n % 100 == 0 {
            return format!("{} hundred", spell(n / 100));
        } else {
            return format!("{} hundred {}", spell(n / 100), spell(n % 100));
        }
    }

    if n < 1_000_000 {
        if n % 1_000 == 0 {
            return format!("{} thousand", spell(n / 1_000));
        } else {
            return format!("{} thousand {}", spell(n / 1_000), spell(n % 1_000));
        }
    }

    if n == 1_000_000 {
        return "one million".to_string();
    }

    unreachable!()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(spell(348), "three hundred forty-eight");
        assert_eq!(spell(9996), "nine thousand nine hundred ninety-six");
        assert_eq!(spell(0), "zero");
        assert_eq!(spell(1001), "one thousand one");
    }
}
