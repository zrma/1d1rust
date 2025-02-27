use crate::read_values_as;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve20254(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (ur, tr, uo, to) = read_values_as!(read_line(reader), i32, i32, i32, i32);

    // 계산 공식: 56 * UR + 24 * TR + 14 * UO + 6 * TO
    let score = 56 * ur + 24 * tr + 14 * uo + 6 * to;

    writeln!(writer, "{}", score).unwrap();
}

// https://www.acmicpc.net/problem/20254
// Site Score
#[test]
fn test_solve20254() {
    struct TestCase {
        s: String,
        want: String,
    }
    for (i, tc) in [
        TestCase {
            s: "1 1 1 1".to_string(),
            want: "100".to_string(),
        },
        TestCase {
            s: "1 10 100 1000".to_string(),
            want: "7696".to_string(),
        },
        TestCase {
            s: "0 0 0 0".to_string(),
            want: "0".to_string(),
        },
        TestCase {
            s: "10 20 30 40".to_string(),
            want: "1700".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = tc.s.as_bytes();
        let mut writer = vec![];
        solve20254(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got.trim(), tc.want.trim(), "failed at {} with {}", i, tc.s);
    }
}
