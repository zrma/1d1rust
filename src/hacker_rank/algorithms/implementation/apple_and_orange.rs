use std::borrow::Borrow;

#[allow(dead_code)]
fn count_apples_and_oranges(
    s: i32,
    t: i32,
    a: i32,
    b: i32,
    apples: Vec<i32>,
    oranges: Vec<i32>,
) -> (i32, i32) {
    let contain_from = |origin: i32| -> Box<dyn Fn(i32) -> bool> {
        Box::new(move |distance: i32| -> bool {
            (s <= origin - distance) && (origin + distance <= t)
        })
    };

    (
        count(apples.borrow(), contain_from(a)),
        count(oranges.borrow(), contain_from(b)),
    )
}

fn count<F: Fn(i32) -> bool>(distances: &[i32], contain: F) -> i32 {
    let mut sum: i32 = 0;
    for &distance in distances.iter() {
        if contain(distance) {
            sum += 1;
        }
    }
    sum
}

// https://www.hackerrank.com/challenges/apple-and-orange/problem
#[test]
fn test_count_apples_and_oranges() {
    let (a, b) = count_apples_and_oranges(7, 11, 5, 15, vec![-2, 2, 1], vec![5, -6]);
    assert_eq!(a, 1);
    assert_eq!(b, 1);
}
