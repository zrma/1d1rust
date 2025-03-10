use crate::read_values_as;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve8723(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (a, b, c) = read_values_as!(read_line(reader), i32, i32, i32);

    let mut v = [a, b, c];
    v.sort();

    if v[0] == v[1] && v[1] == v[2] {
        writeln!(writer, "2").unwrap();
    } else if v[0] * v[0] + v[1] * v[1] == v[2] * v[2] {
        writeln!(writer, "1").unwrap();
    } else {
        writeln!(writer, "0").unwrap();
    }
}

// https://www.acmicpc.net/problem/8723
// noinspection SpellCheckingInspection
// Patyki
#[test]
fn test_solve8723() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "5 12 13".to_string(),
            want: "1".to_string(),
        },
        TestData {
            s: "3 4 5".to_string(),
            want: "1".to_string(),
        },
        TestData {
            s: "1 1 1".to_string(),
            want: "2".to_string(),
        },
        TestData {
            s: "3 3 3".to_string(),
            want: "2".to_string(),
        },
        TestData {
            s: "1 4 5".to_string(),
            want: "0".to_string(),
        },
        TestData {
            s: "1 5 5".to_string(),
            want: "0".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve8723(&mut reader, &mut writer);

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
