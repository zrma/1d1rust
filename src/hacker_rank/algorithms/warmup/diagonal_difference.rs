#[allow(dead_code)]
fn diagonal_difference(arr: Vec<Vec<i32>>) -> i32 {
    let mut val1: i32 = 0;
    let mut val2: i32 = 0;
    let tot_len = arr[0].len() - 1 as usize;

    for (pos, row) in arr.into_iter().enumerate() {
        val1 += row[pos];
        val2 += row[tot_len - pos];
    }

    i32::abs(val1 - val2)
}

// https://www.hackerrank.com/challenges/diagonal-difference/problem
#[test]
fn test_diagonal_difference() {
    let arr = std::vec![
        std::vec![11, 2, 4],
        std::vec![4, 5, 6],
        std::vec![10, 8, -12],
    ];
    let actual = diagonal_difference(arr);
    let expected: i32 = 15;
    assert_eq!(actual, expected);

    let arr = std::vec![
        std::vec![4, 2, 11],
        std::vec![4, 5, 6],
        std::vec![-12, 8, 10],
    ];
    let actual = diagonal_difference(arr);
    let expected: i32 = 15;
    assert_eq!(actual, expected);
}
