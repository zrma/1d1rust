use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve24267(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: i64 = read_line(reader).parse().unwrap();

    writeln!(writer, "{}", n * (n - 1) * (n - 2) / 6).expect("Failed to write");
    writeln!(writer, "3").expect("Failed to write");
}

// https://www.acmicpc.net/problem/24267
// 알고리즘 수업 - 알고리즘의 수행 시간 6
#[test]
fn test_solve24267() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "7".to_string(),
            want: "35\n3\n".to_string(),
        },
        TestData {
            s: "100".to_string(),
            want: "161700\n3\n".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve24267(reader, &mut writer);

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
