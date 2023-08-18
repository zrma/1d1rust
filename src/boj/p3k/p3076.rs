use crate::boj::p3k::p3023::read_coord;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve3076(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (r, c) = read_coord(reader);
    let (a, b) = read_coord(reader);

    for i in 0..r * a {
        for j in 0..c * b {
            if (i / a + j / b) % 2 == 0 {
                write!(writer, "X").unwrap();
            } else {
                write!(writer, ".").unwrap();
            }
        }
        writeln!(writer).unwrap();
    }
}

// https://www.acmicpc.net/problem/3076
// 상근이의 체스판
#[test]
fn test_solve3076() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in vec![
        TestData {
            s: "2 4
2 2"
            .to_string(),
            want: "XX..XX..
XX..XX..
..XX..XX
..XX..XX
"
            .to_string(),
        },
        TestData {
            s: "5 5
2 3"
            .to_string(),
            want: "XXX...XXX...XXX
XXX...XXX...XXX
...XXX...XXX...
...XXX...XXX...
XXX...XXX...XXX
XXX...XXX...XXX
...XXX...XXX...
...XXX...XXX...
XXX...XXX...XXX
XXX...XXX...XXX
"
            .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve3076(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
