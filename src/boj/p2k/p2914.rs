use crate::read_values_as;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve2914(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (cnt, avg) = read_values_as!(read_line(reader), i64, i64);
    // avg = (x + cnt - 1) / cnt
    // avg * cnt = x + cnt - 1
    // avg * cnt - cnt + 1 = x
    // x = cnt * (avg - 1) + 1
    let ans = cnt * (avg - 1) + 1;
    writeln!(writer, "{}", ans).unwrap();
}

// https://www.acmicpc.net/problem/2914
// 저작권
#[test]
fn test_solve2914() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "38 24".to_string(),
            want: "875".to_string(),
        },
        TestData {
            s: "1 100".to_string(),
            want: "100".to_string(),
        },
        TestData {
            s: "10 10".to_string(),
            want: "91".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve2914(&mut reader, &mut writer);

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
