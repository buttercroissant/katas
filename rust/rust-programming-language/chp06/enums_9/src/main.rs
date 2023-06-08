#[derive(Debug)]
enum UsState {
    // Dallas,
    Indiana,
}

enum Coin {
    // Penny,
    // Nickel,
    Quarter(UsState),
}

fn main() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    // With if let usage
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    let mut count = 0;
    let coin1 = Coin::Quarter(UsState::Indiana);

    // match coin1 {
    //     Coin::Quarter(state) => println!("State quarter from {:?}!", state),
    //     _ => count += 1,
    // }

    if let Coin::Quarter(state) = coin1 {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }

    println!("Count: {}", count);
}
