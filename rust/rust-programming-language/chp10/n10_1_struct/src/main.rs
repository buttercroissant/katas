struct Point<T> {
    x: T,
    y: T,
}

struct Point2<T, U> {
    x: T,
    y: U
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    // let wont_work = Point { x: 5, y: 4.0 };
    let will_work = Point2 { x: 5, y: 4.0 };

}
