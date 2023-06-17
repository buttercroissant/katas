// to run binary crate from workspace dir: 
// cargo run/test -p adder

use add_one;
use add_two;

fn main() {
    let num = 5;
    let _addone = add_one::add_one(num);

    println!("Hello, world! {num} plus one is {}!", _addone);

    let _addtwo = add_two::add_two(num);
    println!("Hello, world! {num} plus two is {}!", _addtwo);
}
