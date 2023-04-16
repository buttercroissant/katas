struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

// Multiple impl
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let width = 50;
    let height = 25;

    let rect = Rectangle { width, height };

    let rect2 = Rectangle {
        width: 27,
        height: 33,
    };

    println!("Area of rect is {}", rect.area());
    println!("Can rect hold rect2? {}", rect.can_hold(&rect2));
}
