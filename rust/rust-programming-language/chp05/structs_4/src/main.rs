struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // &self short for self: &Self
    fn area(&self) -> u32 {
        // fn area(&mut self) -> u32 {
        // fn area(self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 30,
    };

    // rect1.width() -> method width
    // rect1.width -> field width
    if rect1.width() {
        println!("The reactangle has a nonzero width, it is {}", rect1.width)
    }

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area(),
    )
}
