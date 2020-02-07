pub fn sum() {
    println!("{}", plus(3, 5));
}

fn plus(a: i32, b: i32) -> i32 {
    a + b
}

#[test]
fn test_plus() {
    assert_eq!(plus(3, 5), 3 + 5)
}
