use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve27323(reader: &mut impl BufRead, writer: &mut impl Write) {
    let a: i32 = read_line(reader).parse().unwrap();
    let b: i32 = read_line(reader).parse().unwrap();

    writeln!(writer, "{}", a * b).unwrap();
}

// https://www.acmicpc.net/problem/27323
// 직사각형
#[test]
fn test_solve27323() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "3
4"
            .to_string(),
            want: "12".to_string(),
        },
        TestData {
            s: "5
5"
            .to_string(),
            want: "25".to_string(),
        },
        TestData {
            s: "100
100"
            .to_string(),
            want: "10000".to_string(),
        },
        TestData {
            s: "2
3"
            .to_string(),
            want: "6".to_string(),
        },
        TestData {
            s: "100
1"
            .to_string(),
            want: "100".to_string(),
        },
        TestData {
            s: "4
4"
            .to_string(),
            want: "16".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve27323(&mut reader, &mut writer);

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
