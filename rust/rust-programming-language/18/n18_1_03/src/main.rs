fn main() {
    let v = vec!['a', 'b', 'c'];

    for (i, val) in v.iter().enumerate() {
        println!("{val} is at index, {i}");
    }
}
