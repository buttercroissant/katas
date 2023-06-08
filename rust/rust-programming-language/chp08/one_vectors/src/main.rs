fn main() {
    let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    // let third: Option<&i32> = v.get(20000);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element"),
    }

    // accessing nonexistent element
    // let does_not_exist = &v[100]; // panic attack
    let does_not_exist = v.get(100); // returns None without panic attack

    let mut v2 = Vec::new();
    v2.push(5);
    v2.push(6);
    v2.push(7);
    v2.pop();

    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    v.push(6);

    // vector might require new memory allocation and copying old elements to new space if insufficient room to put all elements next to each other where vector is currently stored.

    // can't have mutable and immutable references in the same scope
    // println!("The first element is: {first}");

    let v3 = vec![100, 200, 300, 455];
    for i in &v3 {
        println!("i: {i}");
    }

    let mut v4 = vec![725, 825, 925, 1025];
    for k in &mut v4 {
        *k += 50;
        println!("k: {k}");
    }

    // using an enum to store multiple types as vectors can only store values that are the same type
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
