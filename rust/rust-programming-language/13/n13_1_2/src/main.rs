use std::thread;
use std::time::Duration;

fn main() {
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly..");
        thread::sleep(Duration::from_secs(2));
        num
    };

    let example_closure = |x| x;

    let s = example_closure(String::from("hello"));
    // let n = example_closure(5);

    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let only_borrows = || println!("From closure: {:?}", list);

    println!("Before calling closure: {:?}", list);

    only_borrows();

    println!("After calling closure: {:?}", list);


    let mut list2 = vec![4,5,6];
    println!("(Mutably) Before defining closure: {:?}", list2);

    let mut borrows_mutably = || list2.push(7);

    // NOT OK
    // println!("(Mutably) Before calling closure: {:?}", list2);

    borrows_mutably();
    println!("(Mutably) After calling closure: {:?}", list2);
}