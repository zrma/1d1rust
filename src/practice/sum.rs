pub(crate) fn accumulate() {
    println!("{}", range_sum(0, 100));
}

fn range_sum(from: i32, to: i32) -> i32 {
    (from..to).sum()
}

#[test]
fn test_range_sum() {
    assert_eq!(range_sum(0, 11), 55)
}
