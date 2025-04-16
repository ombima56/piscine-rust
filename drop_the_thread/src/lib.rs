use std::cell::{RefCell, Cell};

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Workers {
    pub drops: Cell<usize>,
    pub states: RefCell<Vec<bool>>,
}

impl Workers {
    pub fn new() -> Workers {
        Workers {
            drops: Cell::new(0),
            states: RefCell::new(Vec::new()),
        }
    }

    pub fn track_worker(&self) -> usize {
        self.states.borrow().len()
    }

    pub fn new_worker(&self, c: String) -> (usize, Thread) {
        let id = self.track_worker();
        self.states.borrow_mut().push(false); // false means not dropped
        let thread = Thread::new_thread(id, c, self);
        (id, thread)
    }

    pub fn is_dropped(&self, id: usize) -> bool {
        *self.states.borrow().get(id).unwrap_or(&true)
    }

    pub fn add_drop(&self, id: usize) {
        let mut states = self.states.borrow_mut();
        if let Some(state) = states.get_mut(id) {
            if *state {
                panic!("{} is already dropped", id);
            } else {
                *state = true;
                self.drops.set(self.drops.get() + 1);
            }
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Thread<'a> {
    pub pid: usize,
    pub cmd: String,
    pub parent: &'a Workers,
}

impl<'a> Thread<'a> {
    pub fn new_thread(p: usize, c: String, t: &'a Workers) -> Thread<'a> {
        Thread {
            pid: p,
            cmd: c,
            parent: t,
        }
    }

    pub fn skill(self) {
        drop(self);
    }
}

impl<'a> Drop for Thread<'a> {
    fn drop(&mut self) {
        self.parent.add_drop(self.pid);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drop_the_thread() {
        let worker = Workers::new();
        let (id, thread) = worker.new_worker(String::from("command"));
        thread.skill();
        assert_eq!(worker.is_dropped(id), true);
    }
}
