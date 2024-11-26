use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve13322(reader: &mut impl BufRead, writer: &mut impl Write) {
    let s = read_line(reader);
    let indices = (0..s.len())
        .map(|i| i.to_string())
        .collect::<Vec<_>>()
        .join("\n");

    write!(writer, "{}", indices).expect("Failed to write");
}

// https://www.acmicpc.net/problem/13322
// noinspection SpellCheckingInspection
// 접두사 배열
#[test]
fn test_solve13322() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "ab".to_string(),
            want: "0
1"
            .to_string(),
        },
        TestData {
            s: "banana".to_string(),
            want: "0
1
2
3
4
5"
            .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve13322(&mut reader, &mut writer);

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
