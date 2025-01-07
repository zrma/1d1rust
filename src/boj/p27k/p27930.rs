use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
// noinspection SpellCheckingInspection
fn solve27930(reader: &mut impl BufRead, writer: &mut impl Write) {
    let s = read_line(reader);

    const KOREA: &str = "KOREA";
    const YONSEI: &str = "YONSEI";

    let mut korea_idx = 0;
    let mut yonsei_idx = 0;

    for ch in s.chars() {
        if ch == KOREA.chars().nth(korea_idx).unwrap() {
            korea_idx += 1;
        }
        if ch == YONSEI.chars().nth(yonsei_idx).unwrap() {
            yonsei_idx += 1;
        }

        if korea_idx == KOREA.len() {
            writeln!(writer, "{}", KOREA).unwrap();
            return;
        }
        if yonsei_idx == YONSEI.len() {
            writeln!(writer, "{}", YONSEI).unwrap();
            return;
        }
    }
}

// https://www.acmicpc.net/problem/27930
// noinspection SpellCheckingInspection
// 당신은 운명을 믿나요?
#[test]
fn test_solve27930() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "KOYONSEREAI".to_string(),
            want: "KOREA".to_string(),
        },
        TestData {
            s: "YYOONNSSEEII".to_string(),
            want: "YONSEI".to_string(),
        },
        TestData {
            s: "YKOONRSEEAI".to_string(),
            want: "KOREA".to_string(),
        },
        TestData {
            s: "YKOONRSEIA".to_string(),
            want: "YONSEI".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve27930(&mut reader, &mut writer);

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
