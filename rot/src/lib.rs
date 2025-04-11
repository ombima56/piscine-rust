pub fn rotate(input: &str, key: i8) -> String {
    let key = key.rem_euclid(26) as u8;

    input
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                ((c as u8 - base + key) % 26 + base) as char
            } else {
                c
            }
        })
        .collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn best_rotate() {
        assert_eq!(rotate("a", 26), "a");
        assert_eq!(rotate("m", 0), "m");
        assert_eq!(rotate("m", 13), "z");
        assert_eq!(rotate("a", 15), "p");
        assert_eq!(rotate("MISS", 5), "RNXX");
        assert_eq!(
            rotate("Gur svir obkvat jvmneqf whzc dhvpxyl.", 13),
            "The five boxing wizards jump quickly."
        );
        assert_eq!(
            rotate("Mtb vznhpqd ifky ozrunsl ejgwfx ajc", 5),
            "Ryg aesmuvi nkpd tewzsxq jolbkc foh"
        );
        assert_eq!(
            rotate("Testing with numbers 1 2 3", 4),
            "Xiwxmrk amxl ryqfivw 1 2 3"
        );
        
    }
}
