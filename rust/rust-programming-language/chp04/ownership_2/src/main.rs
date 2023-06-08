// transferring ownership via returning values;
// when a var includes data on the heap goes out of scope, the value will be cleaned up by "drop" unless data ownership has been moved to another variable

fn main() {
    let s1 = gives_ownership(); // gives_ownership moves its return value to s1;

    let s2 = String::from("Hello"); // s2 comes into scope

    let s3 = takes_and_gives_back(s2); // s2 "move" into takes_and_gives_back function
                                       // and which "move" its return value into s3;
} // s3 goes out of scope and "drop"; s2 was "move" so nothing happens; s1 goes out of scope and "drop"

fn gives_ownership() -> String {
    let some_string = String::from("Yours truly"); // some_string comes into scope
    some_string // some_string is returned and "move" out to the calling function
}

// a_string comes into scope in the next line
fn takes_and_gives_back(a_string: String) -> String {
    a_string // a_string being return and "move" out to the calling function
}
