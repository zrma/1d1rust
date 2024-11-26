use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve5532(reader: &mut impl BufRead, writer: &mut impl Write) {
    let total_days: i32 = read_value(read_line(reader));
    let total_math_pages: i32 = read_value(read_line(reader));
    let total_korean_pages: i32 = read_value(read_line(reader));
    let math_pages_per_day: i32 = read_value(read_line(reader));
    let korean_pages_per_day: i32 = read_value(read_line(reader));

    let required_math_days = (total_math_pages as f64 / math_pages_per_day as f64).ceil() as i32;
    let required_korean_days =
        (total_korean_pages as f64 / korean_pages_per_day as f64).ceil() as i32;

    let max_required_days = required_math_days.max(required_korean_days);
    let vacation_remaining = total_days - max_required_days;

    write!(writer, "{}", vacation_remaining).expect("Failed to write");
}

// https://www.acmicpc.net/problem/5532
// noinspection SpellCheckingInspection
// 방학 숙제
#[test]
fn test_solve5532() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "20
25
30
6
8"
            .to_string(),
            want: "15".to_string(),
        },
        TestData {
            s: "15
32
48
4
6"
            .to_string(),
            want: "7".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve5532(&mut reader, &mut writer);

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
