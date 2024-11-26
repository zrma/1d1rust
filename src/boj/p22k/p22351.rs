use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve22351(reader: &mut impl BufRead, writer: &mut impl Write) {
    let s = read_line(reader);
    let s_len = s.len();

    for start_len in 1..=3.min(s_len) {
        let start: usize = s[0..start_len].parse().expect("Should be a number");
        let mut current_num = start;
        let mut generated_string = current_num.to_string();

        while generated_string.len() < s_len {
            current_num += 1;
            generated_string += &current_num.to_string();
        }

        if generated_string == s {
            write!(writer, "{} {}", start, current_num).expect("Failed to write");
            return;
        }
    }
}

// https://www.acmicpc.net/problem/22351
// noinspection SpellCheckingInspection
// 수학은 체육과목 입니다
#[test]
fn test_solve22351() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "9101112".to_string(),
            want: "9 12".to_string(),
        },
        TestData {
            s: "12".to_string(),
            want: "1 2".to_string(),
        },
        TestData {
            s: "1".to_string(),
            want: "1 1".to_string(),
        },
        TestData {
            s: "999".to_string(),
            want: "999 999".to_string(),
        },
        TestData {
            s: "991992993994995996997998999".to_string(),
            want: "991 999".to_string(),
        },
        TestData {
            s: "456789101112131415161718192021".to_string(),
            want: "4 21".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve22351(&mut reader, &mut writer);

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
