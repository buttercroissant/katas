struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// T not of type f32 will not have this method defined
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct Point2<X1, Y1> {
    x: X1,
    y: Y1,
}

// type parameters in struct definition != always in struct's method signatures
impl<X1, Y1> Point2<X1, Y1> {
    // generic params, X2, Y2, only relevant to mixup method
    fn mixup<X2,Y2>(self, other: Point2<X2, Y2>) -> Point2<X1, Y2> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };

    println!("p.x: {}", p.x());

    let p2_1 = Point2 { x: 5, y: 10.4 };
    let p2_2 = Point2 { x: "Hello", y: 'c' };
    let p2_3 = p2_1.mixup(p2_2);

    println!("p2_3.x = {}, p2_3.y = {}", p2_3.x, p2_3.y);
}