#[allow(dead_code)]
fn solve_me_first(a: u32, b: u32) -> u32 {
    a + b
}

// https://www.hackerrank.com/challenges/solve-me-first/problem
#[test]
fn test_solve_me_first() {
    let expected = 5;
    let actual = solve_me_first(2, 3);
    assert_eq!(actual, expected);
}
