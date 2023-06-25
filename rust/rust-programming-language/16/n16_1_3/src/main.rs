use std::thread;

// fn main() {
//     let v = vec![1, 2, 3];

//     let handle = thread::spawn(|| {
//         println!("Here's a vector: {:?}", v);
//     });

//     // drop(v);

//     handle.join().unwrap();
// }

fn main() {
    let v = vec![1, 2, 3];

    // adding 'move' keyword force the closure to take ownership of the value it's using rather than inferring that it should borrow the values
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });
    
    // drop(v); // NOT OK

    handle.join().unwrap();
}
