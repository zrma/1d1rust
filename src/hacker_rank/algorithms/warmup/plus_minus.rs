#[allow(dead_code)]
fn plus_minus(arr: Vec<i32>) -> (f64, f64, f64) {
    let (mut pos, mut nag, mut zero) = (0, 0, 0);
    arr.iter().for_each(|x| match *x {
        n if n > 0 => pos += 1,
        n if n < 0 => nag += 1,
        _ => zero += 1,
    });

    let tot = arr.len() as f64;
    (
        f64::from(pos) / tot,
        f64::from(nag) / tot,
        f64::from(zero) / tot,
    )
}

// https://www.hackerrank.com/challenges/plus-minus/problem
#[test]
fn test_plus_minus() {
    let arr = std::vec![-4, 3, -9, 0, 4, 1];
    let (pos, nag, zero) = plus_minus(arr);
    assert!((pos - 3.0 / 6.0).abs() < f64::EPSILON);
    assert!((nag - 2.0 / 6.0).abs() < f64::EPSILON);
    assert!((zero - 1.0 / 6.0).abs() < f64::EPSILON);
}
