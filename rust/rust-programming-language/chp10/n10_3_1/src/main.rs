// Lifetimes

fn main() {
    // #eg1 - OK
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    // #eg2 - OK
    let string3 = String::from("long string is long");

    {
        let string4 = String::from("quv");
        let result2 = longest(string3.as_str(), string4.as_str());
        println!("The longest string is {}", result2);
    }

    // #eg3 - NOT OK
    // let string5 = String::from("long string is long");
    // let result3;
    // {
    //     let string6 = String::from("xyz");
    //     result3 = longest(string5.as_str(), string6.as_str());
    // }
    // println!("The longest string is {}", result3);
}

// #1 - NOT OK - lifetimes error 
// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x 
//     } else {
//         y
//     }
// }

// #2 - OK - with lifetime annotations in function signatures
// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// #3 - OK - only specified a lifetime param for param x & return type but not for param y
fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

// #4 - NOT OK - value of return goes out of scope at end of function and gets cleaned up
// fn longest<'a>(x: &str, y: &str) -> &'a str {
//     let result = String::from("really long string");
//     result.as_str();
// }

// Lifetime annotation Syntax
// &i32
// &'a i32 - a reference with an explicit lifetime
// &'a mut i32 - a mutable reference with an explicit lifetime