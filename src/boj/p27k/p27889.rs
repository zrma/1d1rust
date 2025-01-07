use crate::utils::io::read_line;
use std::io::{BufRead, Write};

// noinspection SpellCheckingInspection
#[allow(dead_code)]
fn solve27889(reader: &mut impl BufRead, writer: &mut impl Write) {
    let s = read_line(reader);

    let res = match s.as_str() {
        "NLCS" => "North London Collegiate School",
        "BHA" => "Branksome Hall Asia",
        "KIS" => "Korea International School",
        "SJA" => "St. Johnsbury Academy",
        _ => panic!("unexpected input"),
    };

    writeln!(writer, "{}", res).unwrap();
}

// https://www.acmicpc.net/problem/27889
// 특별한 학교 이름
#[test]
fn test_solve27889() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "NLCS".to_string(),
            want: "North London Collegiate School".to_string(),
        },
        TestData {
            s: "BHA".to_string(),
            want: "Branksome Hall Asia".to_string(),
        },
        TestData {
            s: "KIS".to_string(),
            want: "Korea International School".to_string(),
        },
        TestData {
            s: "SJA".to_string(),
            want: "St. Johnsbury Academy".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve27889(&mut reader, &mut writer);

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
