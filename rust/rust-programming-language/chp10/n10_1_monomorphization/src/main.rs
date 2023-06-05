// Monomorphization - process of turning generic code into specific code by filling in concrete types that are used when compiled.

// let integer = Some(5);
// let float = Some(5.0);

// Option<T> is replaced with specific definitions created by the compiler (See below)

enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}

fn main() {
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}
