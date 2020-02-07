pub fn filter(mut v: Vec<i32>) -> Vec<i32> {
    v.retain(|&x| x % 2 != 0);
    v
}

#[test]
fn test_filter() {
    assert_eq!(filter(std::vec![1, 2, 3, 4, 5]), std::vec![1, 3, 5])
}
