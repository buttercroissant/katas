fn main() {
    let mut s = String::new();

    let mut s1 = String::from("foo");
    s1.push_str("bar");
    println!("s1: {s1}");

    let data = "initial contents 1";
    let s2 = data.to_string();
    println!("s2: {s2}");

    let s3 = "initial_contents 2".to_string();
    println!("s3: {s3}");

    let s4 = String::from("initial contents");
    // let h = s4[0];
}
