#[derive(Debug)]
pub struct Student(pub u32, pub String, pub String);

pub fn id(student: &Student) -> u32 {
    student.0
}

pub fn first_name(student: &Student) -> &str {
    &student.1
}

pub fn last_name(student: &Student) -> &str {
    &student.2
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_id() {
        let student = Student(42, "Pedro".to_string(), "Domingo".to_string());
        assert_eq!(id(&student), 42);
    }
    
    #[test]
    fn test_first_name() {
        let student = Student(42, "Alice".to_string(), "Johnson".to_string());
        assert_eq!(first_name(&student), "Alice");
    }
    
    #[test]
    fn test_last_name() {
        let student = Student(42, "Kada".to_string(), "John".to_string());
        assert_eq!(last_name(&student), "John");
    }
}
