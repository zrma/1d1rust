use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve18238(reader: &mut impl BufRead, writer: &mut impl Write) {
    let s = read_line(reader);
    let mut prev = 'A';
    let mut count = 0;

    for curr in s.chars() {
        count += distance_between_chars(prev, curr);
        prev = curr;
    }

    write!(writer, "{}", count).expect("Failed to write");
}

fn distance_between_chars(from: char, to: char) -> i32 {
    let diff = (to as i32 - from as i32).abs();
    diff.min(26 - diff)
}

// https://www.acmicpc.net/problem/18238
// noinspection SpellCheckingInspection
// ZOAC 2
#[test]
fn test_solve18238() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "ZOAC".to_string(),
            want: "26".to_string(),
        },
        TestData {
            s: "LBOLVUEEPMOIENMG".to_string(),
            want: "100".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve18238(&mut reader, &mut writer);

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
