// fn main() {
//     let s1 = String::from("hello");

//     let (s2, len) = calculate_length(s1);

//     println!("The length of '{}' is {}.", s2, len);
// }

// fn calculate_length(s: String) -> (String, usize) {
//     let length = s.len();
//     (s, length)
// }

fn main() {
    let s1 = String::from("hello");

    // &s1 creates a reference to s1 but does not own it.
    let len = calculate_length(&s1);

    println!("The length of '{}' of {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    // s.push_str(", world!"); // unable to modify as its borrowed or immutable
    s.len()
}
// s goes out of scope but does not have ownership so it is not dropped.
// hence, s need not be returned to give back ownership.
// borrowing => action of creating a reference
