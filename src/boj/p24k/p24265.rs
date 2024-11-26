use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve24265(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: i64 = read_line(reader).parse().unwrap();

    writeln!(writer, "{}", n * (n - 1) / 2).expect("Failed to write");
    writeln!(writer, "2").expect("Failed to write");
}

// https://www.acmicpc.net/problem/24265
// 알고리즘 수업 - 알고리즘의 수행 시간 4
#[test]
fn test_solve24265() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "7".to_string(),
            want: "21\n2\n".to_string(),
        },
        TestData {
            s: "100000".to_string(),
            want: "4999950000\n2\n".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve24265(reader, &mut writer);

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
