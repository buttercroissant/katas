// - Rc::clone increases strong_count of an Rc<T> instance
// - Rc<T> instance only cleaned up if strong_count is 0

// Weak references
// - weak reference creation to value within an Rc<T> instance via Rc::downgrade (gets a smart pointer of type Weak<T>)
// - doesn't express an ownership relationship
// - count doesn't affect when an Rc<T> instance is cleaned up
// - won't cause a reference cycle
// - weak references will be broken once strong reference value count is 0

use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}
