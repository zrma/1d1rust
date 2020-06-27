macro_rules! min {
    ($x: expr) => ($x);
    ($x: expr, $($xs: expr), +) => {
        {
            use std::cmp::min;
            min($x, min!($($xs), +))
        }
    }
}

#[test]
fn test_min() {
    assert_eq!(min!(1), 1);
    assert_eq!(min!(2, 1, 3), 1);
    assert_eq!(min!(0, 1, -1), -1);
}

macro_rules! max {
    ($x: expr) => ($x);
    ($x: expr, $($xs: expr), +) => {
        {
            use std::cmp::max;
            max($x, max!($($xs), +))
        }
    }
}

#[test]
fn test_max() {
    assert_eq!(max!(1), 1);
    assert_eq!(max!(1, 3, 2), 3);
    assert_eq!(max!(1, 0, -1), 1);
}

#[allow(dead_code)]
fn bricks_game(arr: Vec<i32>) -> i64 {
    play_sub_game(&arr, 0, arr.len() - 1)
}

fn play_sub_game(arr: &[i32], begin: usize, end: usize) -> i64 {
    let range = end as i32 - begin as i32;
    if range <= 3 {
        let mut sum: i64 = 0;
        for i in 0..(range + 1) {
            if i == 3 {
                break;
            }
            sum += arr[begin + i as usize] as i64;
        }
        sum
    } else {
        let v0 = min!(
            arr[begin] as i64 + play_sub_game(arr, begin + 2, end),
            arr[begin] as i64 + play_sub_game(arr, begin + 3, end),
            arr[begin] as i64 + play_sub_game(arr, begin + 4, end)
        );
        let v1 = min!(
            arr[begin] as i64 + arr[begin + 1] as i64 + play_sub_game(arr, begin + 3, end),
            arr[begin] as i64 + arr[begin + 1] as i64 + play_sub_game(arr, begin + 4, end),
            arr[begin] as i64 + arr[begin + 1] as i64 + play_sub_game(arr, begin + 5, end)
        );
        let v2 = min!(
            arr[begin] as i64
                + arr[begin + 1] as i64
                + arr[begin + 2] as i64
                + play_sub_game(arr, begin + 4, end),
            arr[begin] as i64
                + arr[begin + 1] as i64
                + arr[begin + 2] as i64
                + play_sub_game(arr, begin + 5, end),
            arr[begin] as i64
                + arr[begin + 1] as i64
                + arr[begin + 2] as i64
                + play_sub_game(arr, begin + 6, end)
        );
        max!(v0, v1, v2)
    }
}

// https://www.hackerrank.com/challenges/play-game/problem
#[test]
fn test_bricks_game() {
    struct TestData {
        arr: Vec<i32>,
        expected: i64,
    }
    for data in std::vec![
        TestData {
            arr: vec![1, 2, 3],
            expected: 6
        },
        TestData {
            arr: vec![1, 2, 3, 4],
            expected: 6
        },
        TestData {
            arr: vec![1, 2, 3, 4, 5],
            expected: 6
        },
        TestData {
            arr: vec![999, 1, 1, 1, 0],
            expected: 1001
        },
        TestData {
            arr: vec![0, 1, 1, 1, 999],
            expected: 999
        },
        TestData {
            arr: vec![0, 1, 1, 1, 999, 999],
            expected: 1001
        },
    ] {
        let actual = bricks_game(data.arr);
        assert_eq!(actual, data.expected);
    }
}
