use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve28691(reader: &mut impl BufRead, writer: &mut impl Write) {
    let s = read_line(reader);
    let ans = match s.as_str() {
        "M" => "MatKor",
        "W" => "WiCys",
        "C" => "CyKor",
        "A" => "AlKor",
        "$" => "$clear",
        _ => "",
    };

    writeln!(writer, "{}", ans).unwrap();
}

// https://www.acmicpc.net/problem/28691
// noinspection SpellCheckingInspection
// 정보보호학부 동아리 소개
#[test]
fn test_solve28691() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "M".to_string(),
            want: "MatKor".to_string(),
        },
        TestData {
            s: "W".to_string(),
            want: "WiCys".to_string(),
        },
        TestData {
            s: "C".to_string(),
            want: "CyKor".to_string(),
        },
        TestData {
            s: "A".to_string(),
            want: "AlKor".to_string(),
        },
        TestData {
            s: "$".to_string(),
            want: "$clear".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve28691(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(
            got.trim(),
            data.want.trim(),
            "failed at {} with {}",
            i,
            data.s
        );
    }
}
