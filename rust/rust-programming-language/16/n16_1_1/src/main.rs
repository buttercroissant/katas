// process -> thread(s)

use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("Hi, number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("Hi, number MAIN-{} from the MAIN thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}
