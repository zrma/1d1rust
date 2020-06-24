#[allow(dead_code)]
fn birthday_cake_candles(arr: Vec<i32>) -> i32 {
    let (mut cnt, mut max) = (0, std::i32::MIN);
    for n in arr {
        if n > max {
            cnt = 1;
            max = n;
            continue;
        }
        if n == max {
            cnt += 1;
        }
    }

    cnt
}

// https://www.hackerrank.com/challenges/birthday-cake-candles/problem
#[test]
fn test_birthday_cake_candles() {
    let arr = std::vec![3, 2, 1, 3];
    let actual = birthday_cake_candles(arr);
    let expected: i32 = 2;
    assert_eq!(actual, expected);
}
