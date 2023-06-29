fn main() {
    let x = 5;

    match x {
        // 1|2|3|4|5 => println!("one through 5"),
        1..=5 => println!("one through 5"),
        _ => println!("something else"),
    }

    let a = 'c';
    match a {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
}
