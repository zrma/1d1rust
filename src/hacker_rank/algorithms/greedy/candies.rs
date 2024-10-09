use std::cmp::max;

#[allow(dead_code)]
fn candies(n: usize, arr: Vec<i32>) -> i64 {
    let result = vec![0; n];

    let result = forward(result, &arr);
    let result = backward(result, &arr);

    result.iter().map(|&i| i64::from(i)).sum()
}

fn forward(mut result: Vec<i32>, arr: &[i32]) -> Vec<i32> {
    let mut candy: i32 = 1;
    result[0] = candy;
    for i in 1..result.len() {
        if arr[i] > arr[i - 1] {
            candy += 1;
        } else {
            candy = 1;
        }
        result[i] = candy;
    }
    result
}

fn backward(mut result: Vec<i32>, arr: &[i32]) -> Vec<i32> {
    let mut candy: i32 = 1;
    for i in (0..result.len() - 1).rev() {
        if arr[i] > arr[i + 1] {
            candy += 1;
        } else {
            candy = 1;
        }
        result[i] = max(result[i], candy);
    }
    result
}

// https://www.hackerrank.com/challenges/candies/problem
#[test]
fn test_candies() {
    struct TestData {
        arr: Vec<i32>,
        expected: i64,
    }
    for data in [
        TestData {
            arr: vec![1, 2, 2],
            expected: 4,
        },
        TestData {
            arr: vec![2, 4, 2, 6, 1, 7, 8, 9, 2, 1],
            expected: 19,
        },
        TestData {
            arr: vec![2, 4, 3, 5, 2, 6, 4, 5],
            expected: 12,
        },
        TestData {
            arr: vec![1, 3, 3, 3, 2, 1],
            expected: 10,
        },
        TestData {
            arr: vec![1, 3, 3, 3, 2, 2, 2, 3, 3, 3, 1],
            expected: 15,
        },
    ] {
        let actual = candies(data.arr.len(), data.arr);
        assert_eq!(actual, data.expected);
    }
}
