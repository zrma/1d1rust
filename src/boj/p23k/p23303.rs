use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve23303(reader: &mut impl BufRead, writer: &mut impl Write) {
    let s = read_line(reader);
    let chars = s.chars().collect::<Vec<_>>();

    for i in 1..chars.len() {
        if chars[i] == '2' && (chars[i - 1] == 'D' || chars[i - 1] == 'd') {
            write!(writer, "D2").unwrap();
            return;
        }
    }

    write!(writer, "unrated").unwrap();
}

// https://www.acmicpc.net/problem/23303
// noinspection SpellCheckingInspection
// 이 문제는 D2 입니다.
#[test]
fn test_solve23303() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "naver d2".to_string(),
            want: "D2".to_string(),
        },
        TestData {
            s: "naver d3".to_string(),
            want: "unrated".to_string(),
        },
        TestData {
            s: "".to_string(),
            want: "unrated".to_string(),
        },
        TestData {
            s: "D3D2D3".to_string(),
            want: "D2".to_string(),
        },
        TestData {
            s: "D3d2D3".to_string(),
            want: "D2".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve23303(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
