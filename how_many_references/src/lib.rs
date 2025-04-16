pub use std::rc::Rc;

pub struct Node {
    pub ref_list: Vec<Rc<String>>,
}

impl Node {
    pub fn new(ref_list: Vec<Rc<String>>) -> Node {
        Node { ref_list: ref_list }
    }
    pub fn add_element(&mut self, element: Rc<String>) {
        self.ref_list.push(element);
    }

    pub fn rm_all_ref(&mut self, element: Rc<String>) {
        self.ref_list.retain(|x| !Rc::ptr_eq(x, &element));
    }
}

pub fn how_many_references(ref_list: &Rc<String>) -> usize {
    Rc::strong_count(ref_list)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_how_many_references() {
        let ref_list = Rc::new("hello".to_string());
        assert_eq!(how_many_references(&ref_list), 1);
    }
    #[test]
    fn test_how_many_references_2() {
        let ref_list = Rc::new("hello".to_string());
        let _node = Node::new(vec![ref_list.clone()]);
        assert_eq!(how_many_references(&ref_list), 2);        
    }

    #[test]
    fn test_how_many_references_3() {
        let ref_list = Rc::new("hello".to_string());
        let mut node = Node::new(vec![ref_list.clone()]);
        node.rm_all_ref(ref_list.clone());
        assert_eq!(how_many_references(&ref_list), 1);        
    }
}
