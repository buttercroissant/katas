use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // allows for all threads to finish
    // handle.join().unwrap(); // blocks the current thread running until completion/terminated

    for i in 1..6 {
        println!("hi number {} from the MAIN thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // allows for all threads to finish
    handle.join().unwrap(); 
}