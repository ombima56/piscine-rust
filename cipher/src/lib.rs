#[derive(Debug, PartialEq)]

pub struct CipherError {
   pub expected: String,
}

pub fn cipher(original: &str, ciphered: &str) -> Result<(), CipherError> {
    let expected = alpha_mirror(original);
    if expected == ciphered {
        Ok(())
    }else {
        Err(CipherError{expected})
    }
}

pub fn alpha_mirror(input: &str) -> String{
    input.chars().map(|c| {
        match c {
            'a'..='z' => (b'z' - (c as u8 - b'a')) as char,
            'A'..='Z' => (b'Z' - (c as u8 - b'A')) as char,
            _ => c,
        }
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cipher_ok() {
        let result = cipher("abc", "zyx");
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn test_cipher_err() {
        let result = cipher("abc", "zzz");
        assert_eq!(result, Err(CipherError { expected: "zyx".to_string() }));
    }
}