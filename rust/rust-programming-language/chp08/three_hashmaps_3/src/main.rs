use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("scores_1: {:?}", scores);

    // entry method to only insert if key does not already have a value
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("scores_2: {:?}", scores);

    // updating a value based on the old value
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0); // or_insert returns a mutable reference
        *count += 1; // * deferencing count to assign to that value
    }

    println!("{:?}", map);
}
