pub fn to_url(s: &str) -> String {
    s.replace(" ", "%20")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_url() {
        assert_eq!(to_url("Hello, world!"), "Hello,%20world!");
        assert_eq!(to_url("Rust is awesome"), "Rust%20is%20awesome");
        assert_eq!(to_url("NoSpacesHere"), "NoSpacesHere");
        assert_eq!(to_url(" "), "%20");
        assert_eq!(to_url("Multiple   spaces"), "Multiple%20%20%20spaces");
    }
}
