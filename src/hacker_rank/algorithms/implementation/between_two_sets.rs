#[allow(dead_code)]
fn get_total_x(arr1: &mut [i32], arr2: &mut [i32]) -> i32 {
    arr1.sort_unstable();
    arr2.sort_unstable();

    let begin = arr1[arr1.len() - 1];
    let end = arr2[0];

    let cond1 = |num: i32, arr: &[i32]| arr.iter().all(|&n| -> bool { num % n == 0 });
    let cond2 = |num: i32, arr: &[i32]| arr.iter().all(|&n| -> bool { n % num == 0 });

    let mut cnt: i32 = 0;
    for n in begin..(end + 1) {
        if cond1(n, arr1) && cond2(n, arr2) {
            cnt += 1;
        }
    }
    cnt
}

#[cfg(test)]
use std::borrow::BorrowMut;

// https://www.hackerrank.com/challenges/between-two-sets/problem
#[test]
fn test_get_total_x() {
    {
        let actual = get_total_x(vec![2, 4].borrow_mut(), vec![16, 32, 96].borrow_mut());
        let expected: i32 = 3;
        assert_eq!(actual, expected);
    }

    {
        let actual = get_total_x(vec![3, 4].borrow_mut(), vec![24, 48].borrow_mut());
        let expected: i32 = 2;
        assert_eq!(actual, expected);
    }
}
