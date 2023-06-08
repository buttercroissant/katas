// // Immutability
// fn main() {
//     const ONE_HOUR_IN_SECONDS: u32 = 60 * 60 * 3; // Immutable by default plus required type annotation
//     println!("There are {ONE_HOUR_IN_SECONDS} seconds in an hour.");

//     let x = 5; // Immutable by default
//     println!("The value of x is {x}");

//     x = 7; // Error assigning twice to a immutable variable
//     println!("The value of x is {x}");
// }

// // Mutability
// fn main() {
//     let mut x = 6; // with mut keyword
//     println!("The value of x is {x}");

//     x = 8;
//     println!("The value of x is {x}");
// }

// // Shadowing
// fn main() {
//     let x = 11;
//     println!("The value of x is {x}");

//     let x = x + 15;
//     println!("The value of x is {x}");

//     let x = 999;
//     println!("The value of x is {x}");

//     let x = "test";
//     let x = x.len();
//     println!("The value of x is {x}");
// }
