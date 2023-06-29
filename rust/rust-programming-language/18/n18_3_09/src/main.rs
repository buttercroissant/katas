fn main() {
    let s = Some(String::from("Hello!"));

    // if let Some(_s) = s { // not ok: _s binds the value to the variable
    if let Some(_) = s {
        println!("found a string");
    }

    println!("{:?}", s);
}