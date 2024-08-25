pub(crate) fn filter() {
    for x in filter_odd_numbers(vec![1, 2, 3, 4, 5]) {
        println!("{}", x)
    }

    for x in filter_even_numbers(vec![1, 2, 3, 4, 5]) {
        println!("{}", x)
    }
}

fn filter_odd_numbers(mut v: Vec<i32>) -> Vec<i32> {
    v.retain(|&x| x % 2 != 0);
    v
}

fn filter_even_numbers(mut v: Vec<i32>) -> Vec<i32> {
    v.retain(|&x| x % 2 == 0);
    v
}

#[test]
fn test_filter() {
    assert_eq!(filter_odd_numbers(vec![1, 2, 3, 4, 5]), vec![1, 3, 5]);
    assert_eq!(filter_even_numbers(vec![1, 2, 3, 4, 5]), vec![2, 4]);
}
