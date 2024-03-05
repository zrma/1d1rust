use crate::read_values_as;
use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve13420(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n = read_value(read_line(reader));
    for _ in 0..n {
        let (a, op, b, _, c) = read_values_as!(read_line(reader), i64, String, i64, String, i64);

        let ans = match op.as_str() {
            "+" => a + b == c,
            "-" => a - b == c,
            "*" => a * b == c,
            "/" => a / b == c,
            _ => panic!("unexpected operator"),
        };

        writeln!(writer, "{}", if ans { "correct" } else { "wrong answer" })
            .expect("Failed to write");
    }
}

// https://www.acmicpc.net/problem/13420
// 사칙연산
#[test]
fn test_solve13420() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [TestData {
        s: "4
3 * 2 = 6
11 + 11 = 11
7 - 9 = -2
3 * 0 = 0"
            .to_string(),
        want: "correct
wrong answer
correct
correct
"
        .to_string(),
    }]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve13420(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(got, data.want, "failed at {}th case", i);
    }
}
