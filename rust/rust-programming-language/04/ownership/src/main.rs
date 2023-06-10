// known size at compile time are stored on the stack. e.g. integers type, string literal;

fn main() {
    let s = "hello";

    {
        let v = "im valid here";
        println!("Val of v is {v}");
    }

    // println!("Val of v is {v}"); // variable, v out of scope
    println!("Val of s is {s}");

    string_type();

    let a = 5;
    let b = a;
    println!("a:{}, b:{}", a, b);

    let s1 = String::from("hello String");
    // let s2 = s1; // s1 out of scope; gets invalidated; s1 "move" into s2
    let s2 = s1.clone(); // deep copy
    println!("s1:{} s2:{}", s1, s2);

    let n1 = 5;
    // n1 not "move" to n2;
    let n2 = n1; // see copy trait
    println!("n1:{} n2:{}", n1, n2);

    let u = String::from("hello"); // u comes into scope
    takes_ownership(u); // u's value moves into the function & so, no longer valid here

    let z = 5; // z comes into scope
    makes_copy(z); // z moves into function; okay to still use z afterward as i32 is Copy;
} // z goes out of scope; nothing happens to u as u was moved into takes_ownership function;

fn string_type() {
    // string literal -> stack?
    // String type -> heap?
    let mut s = String::from("hello");

    s.push_str(", world!");
    println!("{}", s);
}

// some_string comes into scope below
fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} // some_string goes out of scope & "drop" is called; Backing memory is freed;

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
} //some_integer goes out of scope; nothing happens;
