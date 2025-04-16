#[derive(Debug)]
pub struct WorkEnvironment {
    pub grade: Link,
}

pub type Link = Option<Box<Worker>>;

#[derive(Debug)]
pub struct Worker {
    pub role: String,
    pub name: String,
    pub next: Link,
}

impl WorkEnvironment {
    pub fn new() -> WorkEnvironment {
        WorkEnvironment { grade: None }
    }

    pub fn add_worker(&mut self, role: String, name: String) {
        let new_worker = Worker {
            role,
            name,
            next: self.grade.take(),
        };
        self.grade = Some(Box::new(new_worker));
    }

    pub fn remove_worker(&mut self) -> Option<String> {
        if let Some(worker) = self.grade.take() {
            self.grade = worker.next;
            return Some(worker.name);
        }
        None
    }

    pub fn last_worker(&self) -> Option<(String, String)> {
        if let Some(worker) = &self.grade {
            return Some((worker.name.clone(), worker.role.clone()));
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_environment_is_empty() {
        let env = WorkEnvironment::new();
        assert!(env.grade.is_none());
    }

    #[test]
    fn test_add_worker_assigns_correct_name_and_role() {
        let mut env = WorkEnvironment::new();
        env.add_worker("CEO".to_string(), "Marie".to_string());

        let grade_ref = env.grade.as_ref();
        assert!(grade_ref.is_some());

        let worker = grade_ref.unwrap();
        assert_eq!(worker.name, "Marie");
        assert_eq!(worker.role, "CEO");
        assert!(worker.next.is_none());
    }

    #[test]
    fn test_remove_worker_returns_name_and_shifts_list() {
        let mut env = WorkEnvironment::new();
        env.add_worker("CEO".to_string(), "Marie".to_string());
        env.add_worker("Manager".to_string(), "Monica".to_string());

        let removed = env.remove_worker();
        assert_eq!(removed, Some("Monica".to_string()));

        let remaining = env.grade.as_ref().unwrap();
        assert_eq!(remaining.name, "Marie");
        assert_eq!(remaining.role, "CEO");
    }

    #[test]
    fn test_last_worker_returns_top_of_stack() {
        let mut env = WorkEnvironment::new();
        env.add_worker("CTO".to_string(), "Luke".to_string());

        let last = env.last_worker();
        assert!(last.is_some());

        let (name, role) = last.unwrap();
        assert_eq!(name, "Luke");
        assert_eq!(role, "CTO");
    }
}
