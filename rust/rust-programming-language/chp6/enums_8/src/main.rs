fn add_fancy_hat() {
    println!("Add fancy hat.")
}

fn remove_fancy_hat() {
    println!("Remove fancy hat.")
}

fn move_player(num_spaces: u8) {
    println!("Move player by {} spaces.", num_spaces)
}

fn reroll() {
    println!("Reroll.")
}

fn main() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // Catch all
        // other => move_player(other),
        // _ => reroll(),
        _ => (),
    }
}
