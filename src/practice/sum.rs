pub fn sum() {
    println!("{}", plus(3, 5));
    println!("{}", agg(0, 100));
}

fn plus(a: i32, b: i32) -> i32 {
    a + b
}

fn agg(from: i32, to: i32) -> i32 {
    (from..to).sum()
}

#[test]
fn test_plus() {
    assert_eq!(plus(3, 5), 3 + 5)
}

#[test]
fn test_agg() {
    assert_eq!(agg(0, 11), 55)
}
