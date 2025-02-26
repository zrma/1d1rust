use crate::read_values_as;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve18301(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (n1, n2, n12) = read_values_as!(read_line(reader), i32, i32, i32);

    // Chapman의 추정법을 이용한 전체 쥐의 개체 수 계산
    // ⌊((n1 + 1)(n2 + 1)/(n12 + 1)) - 1⌋
    let population = ((n1 + 1) * (n2 + 1)) / (n12 + 1) - 1;

    writeln!(writer, "{}", population).unwrap();
}

// https://www.acmicpc.net/problem/18301
// Rats
#[test]
fn test_solve18301() {
    struct TestCase {
        s: String,
        want: String,
    }
    for (i, tc) in [
        TestCase {
            s: "15 18 11".to_string(),
            want: "24".to_string(),
        },
        TestCase {
            s: "1 1 0".to_string(),
            want: "3".to_string(),
        },
        TestCase {
            s: "100 200 50".to_string(),
            want: "397".to_string(),
        },
        TestCase {
            s: "50 50 1".to_string(),
            want: "1299".to_string(),
        },
        TestCase {
            s: "10 10 10".to_string(),
            want: "10".to_string(),
        },
        TestCase {
            s: "0 10 0".to_string(),
            want: "10".to_string(),
        },
        TestCase {
            s: "0 0 0".to_string(),
            want: "0".to_string(),
        },
        TestCase {
            s: "1000 2000 10".to_string(),
            want: "182090".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = tc.s.as_bytes();
        let mut writer = vec![];
        solve18301(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got.trim(), tc.want.trim(), "failed at {} with {}", i, tc.s);
    }
}
