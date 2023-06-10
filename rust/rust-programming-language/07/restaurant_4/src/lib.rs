mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// "use" creates shortcut only for the particular scope in which it occurs
use crate::front_of_house::hosting;

// unidiomatic way
// use crate::front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist()
}

// NOT OK: Compilation error
// mod customer {
//     pub fn eat_at_restaurant() {
//         // diff. scope than the "use" statement
//         hosting::add_to_waitlist();
//     }
// }

// OK
mod customer {
    use crate::front_of_house::hosting;
    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }
}
