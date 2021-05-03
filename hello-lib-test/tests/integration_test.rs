extern crate hello_lib_test;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, hello_lib_test::add_two(2));
}
