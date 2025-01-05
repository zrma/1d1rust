use crate::read_values_as;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve14579(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (a, b) = read_values_as!(read_line(reader), u64, u64);
    let mut ans = 1;
    for i in a..=b {
        ans *= i * (i + 1) / 2;
        ans %= 14_579;
    }

    writeln!(writer, "{}", ans).unwrap();
}

// https://www.acmicpc.net/problem/14579
// 덧셈과 곱셈
#[test]
fn test_solve14579() {
    struct TestData {
        s: String,
        want: String,
    }

    for (i, data) in [
        TestData {
            s: "1 1".to_string(),
            want: "1".to_string(),
        },
        TestData {
            s: "1 2".to_string(),
            want: "3".to_string(),
        },
        TestData {
            s: "1 3".to_string(),
            want: "18".to_string(),
        },
        TestData {
            s: "3 5".to_string(),
            want: "900".to_string(),
        },
        TestData {
            s: "1 238".to_string(),
            want: "0".to_string(),
        },
        TestData {
            s: "1 999".to_string(),
            want: "0".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve14579(&mut reader, &mut writer);

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
