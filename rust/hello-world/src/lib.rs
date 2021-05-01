// The &'static here means the return type has a static lifetime.
// This is a Rust feature that you don't need to worry about now.
pub fn hello() -> &'static str {
    "Hello, World!"
}

#[test]
fn test_output_from_hello() {
    assert_eq!(hello(), "Hello, World!")
}
