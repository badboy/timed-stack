#![cfg_attr(feature = "nightly", allow(unstable_features))]
#![cfg_attr(feature = "nightly", feature(plugin))]
#![cfg_attr(feature = "nightly", plugin(clippy))]
#![cfg_attr(feature = "nightly", allow(let_and_return))]

use std::sync::{Condvar, Mutex};
use std::cell::RefCell;

pub struct TimedStack<T> {
    queue: Mutex<RefCell<Vec<T>>>,
    resource: Condvar,
}

unsafe impl<T: Send> Send for TimedStack<T> {}

impl<T> TimedStack<T> {
    pub fn new() -> TimedStack<T> {
        TimedStack {
            queue: Mutex::new(RefCell::new(Vec::new())),
            resource: Condvar::new(),
        }
    }

    pub fn push(&self, obj: T) -> bool {
        let queue = self.queue.lock().unwrap();
        queue.borrow_mut().push(obj);
        self.resource.notify_all();
        true
    }

    pub fn len(&self) -> usize {
        let queue = self.queue.lock().unwrap();
        let l = queue.borrow().len();
        l
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn pop(&self, timeout_ms: u32) -> Option<T> {
        let mut queue = self.queue.lock().unwrap();

        loop {
            {
                let mut vec = queue.borrow_mut();
                if vec.len() > 0 {
                    let elem = vec.pop();
                    return elem;
                }
            }

            let (q2, t) = self.resource.wait_timeout_ms(queue, timeout_ms).unwrap();
            if !t {
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
