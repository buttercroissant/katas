use std::fs;
use std::io;
// use std::fs::File;
// use std::io::{self, Read};

fn main() {
    // read_username_from_file();
    // read_username_from_file2();
    // read_username_from_file3();
    read_username_from_file4();
}

// fn read_username_from_file() -> Result<String, io::Error> {
//     let username_file_result = File::open("hello.txt");

//     let mut username_file = match username_file_result {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };

//     let mut username = String::new();

//     match username_file.read_to_string(&mut username) {
//         Ok(_) => Ok(username),
//         Err(e) => Err(e),
//     }
// }

// // with ? operator usage
// // ? placed after a Result value works almost as the match expressions; returns the value inside the Ok; returns an Err from the whole function similar to using return keyword
// fn read_username_from_file2() -> Result<String, io::Error> {
//     let mut username_file = File::open("hello.txt")?;
//     let mut username = String::new();

//     username_file.read_to_string(&mut username)?;
//     Ok(username);
// }

// // with chaining method
// fn read_username_from_file3() -> Result<String, io::Error> {
//     let mut username = String::new();

//     File::open("hello.txt")?.read_to_string(&mut username)?;

//     Ok(username);
// }

fn read_username_from_file4() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
