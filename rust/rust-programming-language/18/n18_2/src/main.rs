#[allow(unused)]
fn main() {
    let some_option_value: std::option::Option<i32> = None;

    // let Some(x) = some_option_value; // not ok

    if let Some(x) = some_option_value {
        println!("heya {x}");
    }  else {
        println!("hello world")
    }

    if let x = 5 { // use let instead
        println!("{x}");
    }
}
