use n11_3;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, n11_3_adder::add_two2(2));
}