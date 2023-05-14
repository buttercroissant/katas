use std::collections::HashMap;

fn main() {
    let field_name = String::from("Favourite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();

    // Types that implement Copy trait, eg. i32, values are copied into the hash map. Owned values like strings will be moved and hash map will be the owner of those values.

    // map.insert(&field_name, &field_value);
    // println!("field_name: {field_name}"); // OK
    // println!("field_value: {field_value}"); // OK

    map.insert(field_name, field_value);
    //     println!("field_name: {field_name}"); // NOT OK
    //     println!("field_value: {field_value}"); // NOT OK
}
