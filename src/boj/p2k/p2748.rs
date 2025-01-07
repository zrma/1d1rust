use crate::boj::p2k::p2747::fibonacci;
use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve2748(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: u128 = read_value(read_line(reader));

    let ans = fibonacci(n);

    writeln!(writer, "{}", ans).unwrap();
}

// https://www.acmicpc.net/problem/2748
// 피보나치 수 2
#[test]
fn test_solve2748() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "10".to_string(),
            want: "55".to_string(),
        },
        TestData {
            s: "50".to_string(),
            want: "12586269025".to_string(),
        },
        TestData {
            s: "90".to_string(),
            want: "2880067194370816120".to_string(),
        },
        TestData {
            s: "100".to_string(),
            want: "354224848179261915075".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve2748(&mut reader, &mut writer);

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
