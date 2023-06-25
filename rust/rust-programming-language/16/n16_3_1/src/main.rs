use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);
    println!("m = {:?}", m);

    // m.lock().unwrap();
    // panic!("ICE CREAM");

    {
        // lock returns a LockResult<MutexGuard> 
        // MutexGuard has got Deref & Drop implementation
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}