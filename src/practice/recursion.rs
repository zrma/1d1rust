extern crate num;

use num::PrimInt;

pub fn recursion() {
    println!("{}", fib(15));

    // 64 is i32, overflow have occurred
    // println!("{}", fib(64));

    let num: i64 = 64;
    println!("{}", fib(num));
}

fn fib<T>(n: T) -> T
where
    T: PrimInt,
{
    fib_recur(n, T::zero(), T::one())
}

fn fib_recur<T>(n: T, a: T, b: T) -> T
where
    T: PrimInt,
{
    if n == T::zero() {
        a
    } else if n == T::one() {
        b
    } else {
        fib_recur(n - T::one(), b, a + b)
    }
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

    let num: i64 = 64;
    let expected = 10_610_209_857_723;
    assert_eq!(fib(num), expected);
}
