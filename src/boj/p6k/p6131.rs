use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve6131(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: i32 = read_value(read_line(reader));
    let count = (1..=500)
        .filter(|a| {
            let b_square = a * a + n; // b^2 = a^2 + n
            let b = (b_square as f64).sqrt() as i32;
            b * b == b_square && b <= 500
        })
        .count();

    writeln!(writer, "{}", count).unwrap();
}

// https://www.acmicpc.net/problem/6131
// 완전 제곱수
#[test]
fn test_solve6131() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "15".to_string(),
            want: "2".to_string(),
        },
        TestData {
            s: "3".to_string(),
            want: "1".to_string(),
        },
        TestData {
            s: "1".to_string(),
            want: "0".to_string(),
        },
        TestData {
            s: "0".to_string(),
            want: "500".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve6131(&mut reader, &mut writer);

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
