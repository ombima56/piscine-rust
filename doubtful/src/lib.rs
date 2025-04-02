pub fn doubtful(s: &mut String) {
    s.push('?');
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_doubtful() {
        let mut s = "hello".to_owned();
        doubtful(&mut s);
        assert_eq!(s, "hello?");

        let mut s2 = "".to_owned();
        doubtful(&mut s2);
        assert_eq!(s2, "?");

        let mut s3 = "camelCase".to_owned();
        doubtful(&mut s3);
        assert_eq!(s3, "camelCase?");

        let mut s4 = "olá!".to_owned();
        doubtful(&mut s4);
        assert_eq!(s4, "olá!?");
    }
}

