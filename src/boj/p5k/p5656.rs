use crate::read_values_as;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve5656(reader: &mut impl BufRead, writer: &mut impl Write) {
    let mut i = 0;
    loop {
        i += 1;

        let (a, op, b) = read_values_as!(read_line(reader), i32, String, i32);

        if op == "E" {
            break;
        }

        let ans = match op.as_str() {
            ">" => a > b,
            ">=" => a >= b,
            "<" => a < b,
            "<=" => a <= b,
            "==" => a == b,
            "!=" => a != b,
            _ => panic!("invalid operator"),
        };

        writeln!(writer, "Case {}: {}", i, ans).expect("Failed to write");
    }
}

// https://www.acmicpc.net/problem/5656
// 비교 연산자
#[test]
fn test_solve5656() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [TestData {
        s: "3 != 3
4 < 4
4 <= 5
3 E 3"
            .to_string(),
        want: "Case 1: false
Case 2: false
Case 3: true
"
        .to_string(),
    }]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve5656(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(got, data.want, "case {} failed", i);
    }
}
