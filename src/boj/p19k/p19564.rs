use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve19564(reader: &mut impl BufRead, writer: &mut impl Write) {
    let s = read_line(reader);
    let mut count = 1;
    let mut prev_char = s.chars().next().unwrap();

    for current_char in s.chars().skip(1) {
        if current_char <= prev_char {
            count += 1;
        }
        prev_char = current_char;
    }

    write!(writer, "{}", count).expect("Failed to write");
}

// https://www.acmicpc.net/problem/19564
// noinspection SpellCheckingInspection
// 반복
#[test]
fn test_solve19564() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "polymath".to_string(),
            want: "6".to_string(),
        },
        TestData {
            s: "xyz".to_string(),
            want: "1".to_string(),
        },
        TestData {
            s: "zyx".to_string(),
            want: "3".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve19564(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(
            got.trim(),
            data.want.trim(),
            "failed at {} with {}",
            i,
            data.s
        );
    }
}
