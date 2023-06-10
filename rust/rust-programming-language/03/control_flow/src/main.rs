fn main() {
    // let number = 3;
    let number = 7;

    // bool expected for condition
    // if number {
    if number < 5 {
        println!("condition met");
    } else if number % 3 == 0 {
        println!("branch condition met");
    } else {
        println!("condition not met");
    }

    // ternary in rust; if is an expression;
    let condition = true;

    // let num = if condition { 5 } else { "six" };
    let num = if condition { 5 } else { 6 };

    println!("The val of number is {num}");
}
