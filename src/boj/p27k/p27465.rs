use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve27465(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: i32 = read_value(read_line(reader));
    let ans = if n <= 3 { 4 } else { n + (n % 2) };
    writeln!(writer, "{}", ans).unwrap();
}

// https://www.acmicpc.net/problem/27465
// 소수가 아닌 수
#[test]
fn test_solve27465() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "10".to_string(),
            want: "10".to_string(),
        },
        TestData {
            s: "11".to_string(),
            want: "12".to_string(),
        },
        TestData {
            s: "1".to_string(),
            want: "4".to_string(),
        },
        TestData {
            s: "2".to_string(),
            want: "4".to_string(),
        },
        TestData {
            s: "3".to_string(),
            want: "4".to_string(),
        },
        TestData {
            s: "4".to_string(),
            want: "4".to_string(),
        },
        TestData {
            s: "1000000000".to_string(),
            want: "1000000000".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve27465(&mut reader, &mut writer);

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
