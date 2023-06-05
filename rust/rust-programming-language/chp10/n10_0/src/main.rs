// #1

// fn main() {
//     let number_list = vec![34, 50, 25, 100, 65];

//     let mut largest = &number_list[0];

//     for number in &number_list {
//         if number > largest {
//             largest = number;
//         }
//     }

//     println!("The largest number is {}", largest);

//     let number_list2 = vec![102, 34, 6000, 89, 54, 2, 43, 8];

//     let mut largest2 = &number_list2[0];

//     for number2 in &number_list2 {
//         if number2 > largest2 {
//             largest2 = number2;
//         }
//     }

//     println!("The largest number2 is {}", largest2);

// }

// #2 with abstraction code

fn largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![10,20,30,40,50];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let number_list2 = vec![11,22,33,44,55];
    let result2 = largest(&number_list2);
    println!("The largest number2 is {}", result2);
}