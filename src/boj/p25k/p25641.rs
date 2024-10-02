use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve25641(reader: &mut impl BufRead, writer: &mut impl Write) {
    let num_chars: usize = read_value(read_line(reader));
    let s = read_line(reader);

    let mut s_cnt = s.chars().filter(|&c| c == 's').count();
    let mut c_cnt = num_chars - s_cnt;

    let mut equal_idx = 0;
    for (i, c) in s.chars().enumerate() {
        if s_cnt == c_cnt {
            equal_idx = i;
            break;
        }

        match c {
            's' => s_cnt -= 1,
            _ => c_cnt -= 1,
        }
    }

    write!(writer, "{}", &s[equal_idx..]).expect("Failed to write");
}

// https://www.acmicpc.net/problem/25641
// noinspection SpellCheckingInspection
// 균형 잡힌 소떡소떡
#[test]
fn test_solve25641() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "7
sttstts"
                .to_string(),
            want: "stts".to_string(),
        },
        TestData {
            s: "4
ttts"
                .to_string(),
            want: "ts".to_string(),
        },
        TestData {
            s: "4
stst"
                .to_string(),
            want: "stst".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve25641(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
