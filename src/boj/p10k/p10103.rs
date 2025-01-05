use crate::read_values_as;
use crate::utils::io::{read_line, read_value};
use std::cmp::Ordering::{Greater, Less};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve10103(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n = read_value(read_line(reader));
    let (mut a, mut b) = (100, 100);

    for _ in 0..n {
        let (x, y) = read_values_as!(read_line(reader), i32, i32);
        match x.cmp(&y) {
            Greater => b -= x,
            Less => a -= y,
            _ => {}
        }
    }

    writeln!(writer, "{}", a).unwrap();
    writeln!(writer, "{}", b).unwrap();
}

// https://www.acmicpc.net/problem/10103
// 주사위 게임
#[test]
fn test_solve10103() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "4
5 6
6 6
4 3
5 2"
            .to_string(),
            want: "94
91"
            .to_string(),
        },
        TestData {
            s: "1
1 1"
            .to_string(),
            want: "100
100"
            .to_string(),
        },
        TestData {
            s: "1
1 2"
            .to_string(),
            want: "98
100"
            .to_string(),
        },
        TestData {
            s: "1
2 1"
            .to_string(),
            want: "100
98"
            .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve10103(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(
            got.trim(),
            data.want.trim(),
            "failed at {} with {}",
            i,
            data.s
        );
    }
}
