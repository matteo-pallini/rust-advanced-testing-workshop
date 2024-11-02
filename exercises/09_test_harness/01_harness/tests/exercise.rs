use basic_harness::add_ten;

fn test_add_ten() {
    assert_eq!(add_ten(5), 15);
}

fn test_add_ten_second() {
    assert_eq!(add_ten(10), 20);
}

fn setup() {
    println!("setup");
}

fn cleanup() {
    println!("cleanup");
}

fn main() {
    setup();
    test_add_ten();
    test_add_ten_second();
    cleanup();
}