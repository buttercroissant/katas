// Test #1

fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    #[test]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(5, value);
    }
}

// Test #2

pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests2 {
    use super::*;

    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }
}

// Test #3

fn it_works() {
    assert_eq!(2+ 2, 4);
}

#[cfg(test)]
mod tests3 { 
    
    #[test]
    #[ignore]
    fn expensive_test() {
        panic!("hello")
    }
}