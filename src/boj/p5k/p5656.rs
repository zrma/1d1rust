use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve5656(reader: &mut impl BufRead, writer: &mut impl Write) {
    let mut i = 0;
    loop {
        i += 1;
        let s = read_line(reader);
        let mut iter = s.split_whitespace();
        let a = iter.next().unwrap().parse::<i32>().unwrap();
        let op = iter.next().unwrap();
        let b = iter.next().unwrap().parse::<i32>().unwrap();

        if op == "E" {
            break;
        }

        let result = match op {
            ">" => a > b,
            ">=" => a >= b,
            "<" => a < b,
            "<=" => a <= b,
            "==" => a == b,
            "!=" => a != b,
            _ => panic!("invalid operator"),
        };

        writeln!(writer, "Case {}: {}", i, result).unwrap();
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
    for (i, data) in vec![TestData {
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

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "case {} failed", i);
    }
}
