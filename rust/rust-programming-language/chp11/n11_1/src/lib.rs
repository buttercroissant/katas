// Test #1

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    // #[test]
    // fn panic_macro_test() {
    //     panic!("Make this test fail purposely");
    // }
}

// Test #2

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
        // self.width < other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests2 {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };

        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_can_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };

        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }
}

// Test #3

pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests3 {
    use super::*;

    #[test]
    fn it_adds_two() {
        // assert_eq!(4, add_two(2));
        assert_ne!(5, add_two(2));
    }
}

// Test #4

pub fn greeting(name: &str) -> String {
    // format!("Hello {}!", name)
    format!("Hello!")
}

#[cfg(test)]
mod tests4 {
    use super::*;
 
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        // assert!(result.contains("Carol"));
        assert!(result.contains("Carol"),
        "Greeting did not contain name, value was `{}`",
        result
    );
    }
}

// Test #5

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        // if value < 1 || value > 100 {
        //     panic!("Guess value must be between 1 and 100, got {}.", value);
        // }

        if value < 1 {
            // panic!("Guess value must be between 1 and 100, got {}.", value);
            panic!("Guess value must be less than or equal to 100, got {}", value);
        } else if value > 100 {
            // panic!("Guess value must be less than or equal to 100, got {}", value);
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests5 {
    use super::*;

    #[test]
    // #[should_panic]
    #[should_panic(expected = "less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
}

// Test #6

#[cfg(test)]
mod tests6 {
    use super::*;

    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
