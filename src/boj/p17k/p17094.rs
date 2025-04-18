use crate::utils::io::{read_line, read_value};
use std::cmp::Ordering::{Equal, Greater, Less};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve17094(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n = read_value(read_line(reader));
    let s = read_line(reader);

    let (x, y) = s.chars().take(n).fold((0, 0), |(x, y), c| match c {
        '2' => (x + 1, y),
        'e' => (x, y + 1),
        _ => (x, y),
    });

    match x.cmp(&y) {
        Less => writeln!(writer, "e").unwrap(),
        Equal => writeln!(writer, "yee").unwrap(),
        Greater => writeln!(writer, "2").unwrap(),
    }
}

// https://www.acmicpc.net/problem/17094
// Serious Problem
#[test]
fn test_solve17094() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "12
222eee222eee"
                .to_string(),
            want: "yee".to_string(),
        },
        TestData {
            s: "3
22e"
            .to_string(),
            want: "2".to_string(),
        },
        TestData {
            s: "3
e2e"
            .to_string(),
            want: "e".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve17094(&mut reader, &mut writer);

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
