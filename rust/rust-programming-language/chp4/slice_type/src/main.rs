// fn main() {
//     let mut s = String::from("Hello world!");

//     let word = first_word(&s);

//     println!("word {}", word);

//     s.clear();
// }

// fn second_word(s: &String) -> (usize, usize) {}
// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }

//     s.len()
// }

// A slice is a kind of reference, so it does not have ownership

fn main() {
    let s = String::from("hello world");

    // let hello = &s[0..5];
    let hello = &s[..5];

    // let len s.len();
    // let world = &s[6..len];
    let world = &s[6..];

    println!("hello: {}", hello);
    println!("world: {}", world);

    let word = first_word(&s);
    println!("the first word is {}", word);

    // let mut s2 = String::from("hello world");
    // let word2 = first_word(&s2);
    // s2.clear();
    // println!("the first word is {}", word2);
}

// &str allows usage of both &String and &str
// fn first_word(s: &str) -> &str {
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
