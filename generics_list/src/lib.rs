#[derive(Clone, Debug)]
pub struct List<T> {
    pub head: Option<Box<Node<T>>>,
}

#[derive(Clone, Debug)]
pub struct Node<T> {
    pub value: T,
    pub next: Option<Box<Node<T>>>,
}

impl<T> List<T> {
    pub fn new() -> List<T> {
        List { head: None }
    }

    pub fn push(&mut self, value: T) {
        let new_node = Box::new(Node {
            value,
            next: self.head.take(),
        });
        self.head = Some(new_node)
    }

    pub fn pop(&mut self) {
        if let Some(node) = self.head.take() {
            self.head = node.next;
        }
    }

    pub fn len(&self) -> usize {
        let mut count = 0;
        let mut current = &self.head;

        while let Some(node) = current {
            count += 1;
            current = &node.next;
        }
        count
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_push_and_len() {
        let mut list = List::new();
        assert_eq!(list.len(), 0);

        list.push(1);
        assert_eq!(list.len(), 1);

        list.push(2);
        assert_eq!(list.len(), 2);

        list.push(3);
        assert_eq!(list.len(), 3);
    }
    #[test]
    fn test_list_pop() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.len(), 3);

        list.pop();
        assert_eq!(list.len(), 2);

        list.pop();
        assert_eq!(list.len(), 1);

        list.pop();
        assert_eq!(list.len(), 0);
    }
    #[test]
    fn test_list_clone() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let cloned_list = list.clone();
        assert_eq!(cloned_list.len(), 3);

        list.pop();
        assert_eq!(list.len(), 2);
        assert_eq!(cloned_list.len(), 3);
    }
}
