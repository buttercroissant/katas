fn main() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Got y = {y}"),
        _ => println!("Default case: x = {:?}", x),
    }

    println!("At the end: x = {:?}, y = {y}", x);
}
