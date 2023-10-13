use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve13420(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n = read_line(reader).parse::<usize>().unwrap();
    for _ in 0..n {
        let s = read_line(reader);
        let mut iter = s.split_whitespace();
        let a = iter.next().unwrap().parse::<i64>().unwrap();
        let op = iter.next().unwrap();
        let b = iter.next().unwrap().parse::<i64>().unwrap();
        iter.next();
        let c = iter.next().unwrap().parse::<i64>().unwrap();

        let ans = match op {
            "+" => a + b == c,
            "-" => a - b == c,
            "*" => a * b == c,
            "/" => a / b == c,
            _ => panic!("unexpected operator"),
        };

        writeln!(writer, "{}", if ans { "correct" } else { "wrong answer" }).unwrap();
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

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "failed at {}th case", i);
    }
}
