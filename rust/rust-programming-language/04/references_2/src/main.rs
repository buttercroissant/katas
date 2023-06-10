// Mutable References

// fn main() {
//     let mut s = String::from("hello");

//     change(&mut s);
// }

// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }

// a mutable reference can have no other references to that value. prevents data race.
// only one mutable references.

// fn main() {
//     let mut s = String::from("hello");

//     let r1 = &mut s;
//     let r2 = &mut s;

//     println!("{} {}", r1, r2);
// }

// fn main() {
//     let mut s = String::from("hello");

//     {
//         let r1 = &mut s;
//     }

//     let r2 = &mut s;
// }

// fn main() {
//     let mut s = String::from("hello");

//     let r1 = &s;
//     let r2 = &s;
//     let r3 = &mut s; // NOT OK

//     println!("{} {} {}", r1, r2, r2);
// }

fn main() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;

    println!("{}, {}", r1, r2);

    let r3 = &mut s; // OK
    println!("{}", r3);
}
// reference's scope starts where it's introduced and valid through last time reference is used.
// r1 & r2 scope don't overlap with r3. So, code compiles
