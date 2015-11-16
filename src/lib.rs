use std::sync::{Condvar, Mutex};
use std::cell::RefCell;

pub struct TimedStack<T> {
    queue: RefCell<Vec<T>>,
}

impl<T> TimedStack<T> {
    pub fn new() -> TimedStack<T> {
        TimedStack {
            queue: RefCell::new(Vec::new()),
        }
    }

    pub fn push(&self, obj: T) -> bool {
        self.queue.borrow_mut().push(obj);
        true
    }

    pub fn len(&self) -> usize {
        self.queue.borrow().len()
    }

    pub fn empty(&self) -> bool {
        self.len() == 0
    }

    pub fn pop(&self) -> Option<T> {
        if self.empty() {
            return None
        }

        self.queue.borrow_mut().pop()
    }
}

#[test]
fn push() {
    let mut t = TimedStack::new();

    assert!(t.push(1));
    assert_eq!(1, t.len());
}

#[test]
fn pop() {
    let mut t = TimedStack::new();
    t.push(1);

    assert_eq!(1, t.len());
    assert_eq!(Some(1), t.pop());
    assert_eq!(0, t.len());
    assert_eq!(None, t.pop());
}
