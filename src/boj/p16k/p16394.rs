use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve16394(reader: &mut impl BufRead, writer: &mut impl Write) {
    let year: i32 = read_value(read_line(reader));

    // 홍익대학교 개교 연도: 1946년
    let hongik_age = year - 1946;

    writeln!(writer, "{}", hongik_age).unwrap();
}

// https://www.acmicpc.net/problem/16394
// 홍익대학교
#[test]
fn test_solve16394() {
    struct TestCase {
        s: String,
        want: String,
    }
    for (i, tc) in [
        TestCase {
            s: "2018".to_string(),
            want: "72".to_string(),
        },
        TestCase {
            s: "1946".to_string(),
            want: "0".to_string(),
        },
        TestCase {
            s: "2000".to_string(),
            want: "54".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = tc.s.as_bytes();
        let mut writer = vec![];
        solve16394(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got.trim(), tc.want.trim(), "failed at {} with {}", i, tc.s);
    }
}
