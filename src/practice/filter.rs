pub(crate) fn filter() {
    for x in filter_vec(vec![1, 2, 3, 4, 5]) {
        println!("{}", x)
    }
}

fn filter_vec(mut v: Vec<i32>) -> Vec<i32> {
    v.retain(|&x| x % 2 != 0);
    v
}

#[test]
fn test_filter() {
    assert_eq!(filter_vec(vec![1, 2, 3, 4, 5]), vec![1, 3, 5])
}
