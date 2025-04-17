#[derive(Clone)]
pub struct StringValue {
    pub value: String,
}

pub trait AppendStr {
    fn append_str(&mut self, str_to_append: String) -> Self;

    fn append_number(&mut self, nb_to_append: f64) -> Self;

    fn remove_punctuation_marks(&mut self) -> Self;
}

impl AppendStr for StringValue {
    fn append_str(&mut self, str_to_append: String) -> Self {
        self.value.push_str(&str_to_append);
        self.clone()
    }

    fn append_number(&mut self, nb_to_append: f64) -> Self {
        self.value.push_str(&nb_to_append.to_string());
        self.clone()
    }

    fn remove_punctuation_marks(&mut self) -> Self {
        let punctuation = ['.', ',', '!', '?', ';', ':'];
        self.value.retain(|c| !punctuation.contains(&c));
        self.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_append_str() {
        let mut str_aux = StringValue {
            value: String::from("hello"),
        };
        str_aux.append_str(String::from(" there!"));
        assert_eq!(str_aux.value, "hello there!");
    }
    #[test]
    fn test_append_number() {
        let mut str_aux = StringValue {
            value: String::from("hello"),
        };
        str_aux.append_number(42.0);
        assert_eq!(str_aux.value, "hello42");
    }
    #[test]
    fn test_remove_punctuation_marks() {
        let mut str_aux = StringValue {
            value: String::from("hello, world!"),
        };
        str_aux.remove_punctuation_marks();
        assert_eq!(str_aux.value, "hello world");
    }
}
