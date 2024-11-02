//! Write an empty entrypoint for the test target defined in `Cargo.toml`.


pub fn add_ten(value: i32) -> i32 {
    value + 10
}

#[test]
fn test_add_ten() {
    assert_eq!(add_ten(5), 15);
}