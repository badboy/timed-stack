#![cfg_attr(feature = "nightly", feature(duration_span))]

extern crate timed_stack;

use std::thread;
use std::thread::sleep;
use std::sync::Arc;
use std::time::Duration;
use timed_stack::TimedStack;

#[test]
fn one_thread() {
    let stack = TimedStack::new();
    let arc_stack = Arc::new(stack);

    let stack2 = arc_stack.clone();
    let thread = thread::spawn(move|| {
        stack2.push(42);
    });

    match arc_stack.pop(500) {
        None => panic!("Pop returned nothing"),
        Some(i) => assert_eq!(i, 42)
    };

    thread.join().unwrap();

    assert_eq!(None, arc_stack.pop(20));
}

#[test]
fn two_threads() {
    let stack = TimedStack::new();
    let arc_stack = Arc::new(stack);

    let stack2 = arc_stack.clone();
    let t1 = thread::spawn(move|| {
        let v = stack2.pop(50).unwrap();
        assert!(v == 42 || v == 50);
    });
    let stack3 = arc_stack.clone();
    let t2 = thread::spawn(move|| {
        let v = stack3.pop(50).unwrap();
        assert!(v == 42 || v == 50);
    });

    arc_stack.push(42);
    sleep(Duration::from_millis(10));
    arc_stack.push(50);

    t1.join().unwrap();
    t2.join().unwrap();
}

#[cfg(feature = "nightly")]
#[test]
fn duration() {
    let stack : TimedStack<u32> = TimedStack::new();

    let d = Duration::span(||{
        assert_eq!(None, stack.pop(50));
    });


    assert_eq!(0, d.as_secs());
    assert!(d.subsec_nanos() > 50*1000*1000);
}
