pub fn add_two(x: i32) -> i32 {
    x + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_add_two() {
        let res = add_two(6);
        assert_eq!(res, 8);
    }
}