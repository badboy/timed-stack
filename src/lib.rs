#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![cfg_attr(feature = "clippy", allow(let_and_return))]

use std::time::Duration;
use std::collections::VecDeque;
use std::sync::{Condvar, Mutex};
use std::cell::RefCell;

pub struct TimedStack<T> {
    queue: Mutex<RefCell<VecDeque<T>>>,
    resource: Condvar,
}

unsafe impl<T: Send> Send for TimedStack<T> {}

impl<T> TimedStack<T> {
    pub fn new() -> TimedStack<T> {
        TimedStack {
            queue: Mutex::new(RefCell::new(VecDeque::new())),
            resource: Condvar::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> TimedStack<T> {
        TimedStack {
            queue: Mutex::new(RefCell::new(VecDeque::with_capacity(capacity))),
            resource: Condvar::new(),
        }
    }

    pub fn push(&self, obj: T) -> bool {
        let queue = self.queue.lock().unwrap();
        queue.borrow_mut().push_back(obj);
        self.resource.notify_all();
        true
    }

    pub fn len(&self) -> usize {
        let queue = self.queue.lock().unwrap();
        let length = queue.borrow().len(); // borrowck is not satisfied if we return this
        length
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn pop(&self, timeout_ms: u64) -> Option<T> {
        let timeout_ms = Duration::from_millis(timeout_ms);
        let mut queue = self.queue.lock().unwrap();

        loop {
            {
                let mut vec = queue.borrow_mut();
                if vec.len() > 0 {
                    let elem = vec.pop_front();
                    return elem;
                }
            }

            let (q2, t) = self.resource.wait_timeout(queue, timeout_ms).unwrap();
            if t.timed_out() {
                break;
            }

            queue = q2;
        }

        None
    }
}

#[test]
fn push() {
    let t = TimedStack::new();

    assert!(t.push(1));
    assert_eq!(1, t.len());
}

#[test]
fn pop() {
    let t = TimedStack::new();
    t.push(1);

    assert_eq!(1, t.len());
    assert_eq!(Some(1), t.pop(1));
    assert_eq!(0, t.len());
    assert_eq!(None, t.pop(1));
}

#[test]
fn with_cap() {
    let t = TimedStack::with_capacity(10);

    assert_eq!(0, t.len());
    t.push(1);
    assert_eq!(1, t.len());
}
