#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut list = [
        Rectangle { width: 10, height: 1},
        Rectangle { width: 10, height: 1},
        Rectangle { width: 10, height: 1},
    ];

    list.sort_by_key(|r| r.width);
    println!("{:#?}", list);

    // let mut list2 = [
    //     Rectangle { width: 10, height: 1},
    //     Rectangle { width: 3, height: 5},
    //     Rectangle { width: 7, height: 12},
    // ];

    // let mut sort_operations = vec![];
    // let value = String::from("by key called");

    // list2.sort_by_key(|r| {
    // // NOT OK
    //     sort_operations.push(value);
    //     r.width
    // });
    // println!("{:#?}", list);

    let mut list3 = [
    Rectangle { width: 10, height: 1},
    Rectangle { width: 3, height: 5},
    Rectangle { width: 7, height: 12},
    ];

    let mut num_sort_operations2 = 0;
    list3.sort_by_key(|r| {
        num_sort_operations2 += 1;
        r.width
    });
    println!("{:#?}, sorted in {num_sort_operations2}", list);

}