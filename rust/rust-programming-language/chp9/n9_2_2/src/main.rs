use std::fs::File;

fn main() {
    //
    // let greeting_file = File::open("hello.txt").unwrap();

    // .expect allows for customize error message
    let greeting_file =
        File::open("hello.txt").expect("hello.txt should be included in this project");
}
