// Dangling References

fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
} // NOT OK
  // s goes out of scope and its dropped. its memory goes away(deallocated).
  // but return a reference of s so this reference would be pointing to an invalid String.

fn no_dangle() -> String {
    let s = String::from("hello");

    s
} // OK
