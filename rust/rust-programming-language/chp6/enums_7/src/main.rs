#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None, // None case has to be handled due to Option usage
        Some(i) => Some(i + 1),
    }
}

fn main() {
    println!("{}", value_in_cents(Coin::Quarter(UsState::Alabama)));
    println!("{}", value_in_cents(Coin::Penny));

    let five = Some(5);
    let size = plus_one(five);
    let none = plus_one(None);
}
