mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// "pub use" re-exports from the root module
pub use crate::front_of_house::hosting;

// restaurant::front_of_house::hosting::add_to_waitlist() <- without "pub use" for external code to call the fn
// restaurant::hosting::add_to_waitlist() <- with "pub use" for external code to call the fn without the full path

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
