use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve10569(reader: &mut impl BufRead, writer: &mut impl Write) {
    let t = read_line(reader).parse::<usize>().unwrap();
    for _ in 0..t {
        let (v, e) = {
            let s = read_line(reader);
            let mut iter = s.split_whitespace();
            (
                iter.next().unwrap().parse::<i32>().unwrap(),
                iter.next().unwrap().parse::<i32>().unwrap(),
            )
        };
        writeln!(writer, "{}", 2 - v + e).unwrap();
    }
}

// https://www.acmicpc.net/problem/10569
// 다면체
// noinspection SpellCheckingInspection
#[test]
fn test_solve10569() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in vec![
        TestData {
            s: "2
8 12
4 6"
            .to_string(),
            want: "6
4
"
            .to_string(),
        },
        TestData {
            s: "1
4 6"
            .to_string(),
            want: "4
"
            .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve10569(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "case {}", i);
    }
}