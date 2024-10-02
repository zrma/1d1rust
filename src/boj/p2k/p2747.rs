use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve2747(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: u128 = read_value(read_line(reader));

    let ans = fibonacci(n);

    write!(writer, "{}", ans).unwrap();
}

pub fn fibonacci(n: u128) -> u128 {
    let (mut prev, mut curr) = (0u128, 1u128);
    for _ in 0..n {
        let next = prev + curr;
        prev = curr;
        curr = next;
    }

    prev
}

// https://www.acmicpc.net/problem/2747
// 피보나치 수
#[test]
fn test_solve2747() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "10".to_string(),
            want: "55".to_string(),
        },
        TestData {
            s: "100".to_string(),
            want: "354224848179261915075".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve2747(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
