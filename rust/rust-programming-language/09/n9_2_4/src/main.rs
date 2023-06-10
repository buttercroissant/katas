use std::error::Error;
use std::fs::File;

// fn main() {
//     // incompatible return type for ? operator usage
//     // let greeting_file = File::open("hello.txt")?;
// }

fn main() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello.txt");
    Ok(())
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
