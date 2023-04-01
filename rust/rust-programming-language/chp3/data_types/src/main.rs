fn main() {
    let x: f32 = 3.22222222222;
    println!("Value of x is {x}");

    // default: f64?
    let y = 3.22222222222222222222222;
    println!("Value of y is {y}");

    let sum = 2 + 3;
    println!("Value of sum is {sum}");

    let difference = 23.555 - 7.7775;
    println!("Value of difference is {difference}");

    let quotient = 50.02 / 48.2;
    println!("Value of quotient is {quotient}");

    // let truncated = -5 / 3.2;
    let truncated = -5 / 3;
    println!("Value of truncated is {truncated}");

    let remainder = 43 % 5;
    println!("Value of remainder is {remainder}");

    let t = true;
    let f: bool = false;
    println!("Bools value: t => {t} & f => {f} ");

    let some_char = 'z';
    let some_word = "someword";
    println!("Char: {some_char}, Word: {some_word}");

    let tup = (500, 5.2, 1);
    let (x, y, z) = tup;
    println!("Tuple: {x} {y} {z}");

    let five_hundred = tup.0;
    let five_point_two = tup.1;
    let one = tup.2;
    println!("Tuple2: {five_hundred} {five_point_two} {one}");

    // arrays => fixed length, must be same types
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    // let first_month = months[0];
    // let arr = [5, 4, 3, 2, 1];
    // let arr2 = [3; 5];

    // let el = months[25];
    // println!("Invalid index access {el}");
}
