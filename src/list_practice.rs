pub fn filter(mut v: Vec<i32>) -> Vec<i32> {
    v.retain(|&x| x % 2 != 0);
    v
}

use std::ops::{Add, Sub};

extern crate num;

use num::FromPrimitive;

pub fn fib<T: Add<Output = T> + Sub<Output = T> + Copy + PartialEq + FromPrimitive>(n: T) -> T {
    fib_recur(
        n,
        FromPrimitive::from_i64(0).expect("0 must be convertible to type of n"),
        FromPrimitive::from_i64(1).expect("1 must be convertible to type of n"),
    )
}

fn fib_recur<T: Add<Output = T> + Sub<Output = T> + Copy + PartialEq + FromPrimitive>(
    n: T,
    a: T,
    b: T,
) -> T {
    if n == FromPrimitive::from_i64(0).expect("0 must be convertible to type of n") {
        a
    } else if n == FromPrimitive::from_i64(1).expect("1 must be convertible to type of n") {
        b
    } else {
        fib_recur(
            n - FromPrimitive::from_i64(1).expect("1 must be convertible to type of n"),
            b,
            a + b,
        )
    }
}

#[test]
fn test_filter() {
    assert_eq!(filter(std::vec![1, 2, 3, 4, 5]), std::vec![1, 3, 5])
}

#[test]
fn test_fib() {
    assert_eq!(fib(0), 0);
    assert_eq!(fib(1), 1);
    assert_eq!(fib(2), 1);
    assert_eq!(fib(3), 2);
    assert_eq!(fib(4), 3);
    assert_eq!(fib(5), 5);
    assert_eq!(fib(6), 8);
}
