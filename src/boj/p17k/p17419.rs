use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve17419(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n = read_value(read_line(reader));
    let bits = read_line(reader);

    let ones_count = bits.chars().take(n).filter(|&ch| ch == '1').count();
    write!(writer, "{}", ones_count).expect("Failed to write");
}

// https://www.acmicpc.net/problem/17419
// noinspection SpellCheckingInspection
// 비트가 넘쳐흘러
#[test]
fn test_solve17419() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "5
10110"
                .to_string(),
            want: "3".to_string(),
        },
        TestData {
            s: "5
11111"
                .to_string(),
            want: "5".to_string(),
        },
        TestData {
            s: "5
00000"
                .to_string(),
            want: "0".to_string(),
        },
        TestData {
            s: "3
101"
            .to_string(),
            want: "2".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve17419(&mut reader, &mut writer);

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
