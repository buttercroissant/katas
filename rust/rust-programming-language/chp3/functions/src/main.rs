// Statements - instructions that perform some action and do not return a value
// Expressions - evaluate to a resultant value

fn main() {
    println!("Hello, world!");

    another_function();

    give_me_five(5);
    give_me_five(10);

    print_labeled_measurement(5, 'm');

    // let x = (let y = 6);
    // let a = (let p = (let p = (let l =  (let e = "apple"))));

    expressions_sample();

    let x = func_with_return_val();
    println!("The value of x is {x}");
}

fn another_function() {
    println!("Another function.");
}

fn give_me_five(x: i32) {
    println!("High {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is {value}{unit_label}");
}

fn expressions_sample() {
    let y = {
        let x = 3;
        x + 1 // expressions don't end with semicolons; otherwise, it becomes a statement and no return a value
    };

    println!("The value of y is {y}");
}

fn func_with_return_val() -> i32 {
    // lonely 9
    9
    // 9;
}
