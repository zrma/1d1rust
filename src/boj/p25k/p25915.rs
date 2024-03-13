use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve25915(reader: &mut impl BufRead, writer: &mut impl Write) {
    let c: char = read_value(read_line(reader));
    let diff = (c as u8).abs_diff(b'I');

    write!(writer, "{}", diff + 84).expect("Failed to write");
}

// https://www.acmicpc.net/problem/25915
// noinspection SpellCheckingInspection
// 연세여 사랑한다
#[test]
fn test_solve25915() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "A".to_string(),
            want: "92".to_string(),
        },
        TestData {
            s: "I".to_string(),
            want: "84".to_string(),
        },
        TestData {
            s: "Z".to_string(),
            want: "101".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve25915(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
