// NOT OK
// x has a lifetime shorter than r; r cannot reference x;
// fn main() {
//     let r;

//     {
//         let x = 5;
//         r = &x;
//     }

//     println!("r: {}", r);
// }

// OK
// x has a lifetime larger than r; r can reference x;
fn main() {
    let x = 5;

    let r = &x;

    println!("r: {}", r);
}