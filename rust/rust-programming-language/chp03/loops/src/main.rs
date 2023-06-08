// fn main() {
//     // loop {
//     //     println!("Hello, world!");
//     // }

//     let mut counter = 0;

//     let res = loop {
//         counter += 1;

//         if counter == 5 {
//             break counter * 2;
//         };

//         println!("Hello, world! Current counter val is {counter}");
//     };

//     println!("Counter val is {res}");
// }

// fn main() {
//     let mut count = 0;
//     'counting_up: loop {
//         println!("count = {count}");
//         let mut remaining = 10;

//         loop {
//             println!("remaining = {remaining}");
//             if remaining == 9 {
//                 break;
//             }
//             if count == 2 {
//                 break 'counting_up;
//             }
//             remaining -= 1;
//         }

//         count += 1;
//     }

//     println!("End count = {count} ");
// }

// fn main() {
//     let mut number = 3;

//     while number != 0 {
//         println!("{number}");
//         number -= 1;
//     }

//     println!("Liftoff!");
// }

// fn main() {
//     let a = [10, 20, 30, 40, 50];

//     for el in a {
//         println!("Value is {el}");
//     }
// }

fn main() {
    // for num in (1..4).rev() {
    for num in 1..4 {
        println!("{num}!");
    }
    println!("LiftOFF!")
}
