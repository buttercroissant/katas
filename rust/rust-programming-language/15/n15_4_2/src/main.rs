enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("Rc count after creating a: {}", Rc::strong_count(&a));

    // Rc::clone increases reference count each call
    let b = Cons(3, Rc::clone(&a)); 
    println!("Rc count after creating b: {}", Rc::strong_count(&a));
    let c = Cons(4, Rc::clone(&a));

    println!("Rc count after creating c: {}", Rc::strong_count(&a));

    {
        let d = Cons(5, Rc::clone(&a));
        println!("Rc count after creating d: {}", Rc::strong_count(&a));
    }
    println!("Rc count after d goes out of scope: {}", Rc::strong_count(&a));
    // Implementation of Drop trait decreases the rc count when an Rc<T> val goes out of scope
}