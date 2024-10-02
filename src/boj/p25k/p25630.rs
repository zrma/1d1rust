use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve25630(reader: &mut impl BufRead, writer: &mut impl Write) {
    let len: usize = read_value(read_line(reader));
    let s = read_line(reader);
    let s = s.as_bytes();

    let change_count = s
        .iter()
        .zip(s.iter().rev())
        .take(len / 2)
        .filter(|(a, b)| a != b)
        .count();

    write!(writer, "{}", change_count).expect("Failed to write");
}

// https://www.acmicpc.net/problem/25630
// noinspection SpellCheckingInspection
// 팰린드롬 소떡소떡
#[test]
fn test_solve25630() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "7
ststtss"
                .to_string(),
            want: "2".to_string(),
        },
        TestData {
            s: "8
ttsststt"
                .to_string(),
            want: "1".to_string(),
        },
        TestData {
            s: "6
ssttss"
                .to_string(),
            want: "0".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve25630(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
