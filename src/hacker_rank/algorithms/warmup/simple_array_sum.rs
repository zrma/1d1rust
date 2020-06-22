#[allow(dead_code)]
fn simple_array_sum(arr: Vec<i32>) -> i32 {
    arr.iter().sum()
}

// https://www.hackerrank.com/challenges/simple-array-sum/problem
#[test]
fn test_simple_array_sum() {
    let arr = std::vec![1, 2, 3, 4, 10, 11];
    let expected = 31;
    let actual = simple_array_sum(arr);
    assert_eq!(actual, expected);
}
