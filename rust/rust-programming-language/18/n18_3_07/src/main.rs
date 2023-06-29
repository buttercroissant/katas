enum Colour {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32},
    Write(String),
    ChangeColour(Colour),
}

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let msg = Message::ChangeColour(Colour::Hsv(0, 160, 255));

    match msg {
        Message::ChangeColour(Colour::Rgb(r, g, b)) => {
            println!("Change colour to red {r}, green {g}, and blue {b}");
        },
        Message::ChangeColour(Colour::Hsv(h, s, v)) => {
            println!("Change colour to hue {h}, saturation {s}, value {v}");
        },
        _ => {},
    }


    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
    println!("{feet}, {inches}, {x}, {y}");
}
