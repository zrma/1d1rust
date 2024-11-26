use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve15886(reader: &mut impl BufRead, writer: &mut impl Write) {
    let _: usize = read_value(read_line(reader));
    let s = read_line(reader);

    let count = s
        .as_bytes()
        .windows(2)
        .filter(|&window| window == b"EW")
        .count();

    write!(writer, "{}", count).expect("Failed to write");
}

// https://www.acmicpc.net/problem/15886
// noinspection SpellCheckingInspection
// 내 선물을 받아줘 2
#[test]
fn test_solve15886() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "6
EEWWEW"
                .to_string(),
            want: "2".to_string(),
        },
        TestData {
            s: "5
EEEEW"
                .to_string(),
            want: "1".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve15886(&mut reader, &mut writer);

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
