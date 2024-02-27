use crate::read_values_as;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve14915(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (n, base) = read_values_as!(read_line(reader), usize, usize);

    if n == 0 {
        write!(writer, "0").unwrap();
        return;
    }

    let mut num = n;
    let mut res = vec![];
    while num > 0 {
        let digit = num % base;
        let c = if digit < 10 {
            (b'0' + digit as u8) as char
        } else {
            (b'A' + (digit - 10) as u8) as char
        };
        res.push(c);
        num /= base;
    }

    res.reverse();
    let output = res.into_iter().collect::<String>();
    write!(writer, "{}", output).unwrap();
}

#[test]
// https://www.acmicpc.net/problem/14915
// 진수 변환기
fn test_solve14915() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "8 2".to_string(),
            want: "1000".to_string(),
        },
        TestData {
            s: "15 4".to_string(),
            want: "33".to_string(),
        },
        TestData {
            s: "248 16".to_string(),
            want: "F8".to_string(),
        },
        TestData {
            s: "10 8".to_string(),
            want: "12".to_string(),
        },
        TestData {
            s: "10 3".to_string(),
            want: "101".to_string(),
        },
        TestData {
            s: "10 11".to_string(),
            want: "A".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve14915(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
