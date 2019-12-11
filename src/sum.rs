pub fn plus(a: i32, b: i32) -> i32 {
    return a + b;
}

#[test]
fn test_plus() {
    assert_eq!(plus(3, 5), 3 + 5)
}
