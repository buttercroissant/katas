enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColour(i32, i32, i32),
}

fn main() {
    let msg = Message::ChangeColour(0, 160, 255);

    match msg {
        Message::Quit => println!("The Quit variant has no data to destructure"),
        Message::Move { x, y } => println!("Move in the x direction {x} and in the y direction {y}"),
        Message::Write(text) => println!("Text message: {text}"),
        Message::ChangeColour(r, g, b) => println!("Change the colour to red {r}, green {g}, and blue {b}"),
    }
}