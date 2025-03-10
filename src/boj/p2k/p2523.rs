use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve2523(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n = read_value(read_line(reader));

    let ans = (1..=n)
        .chain((1..n).rev())
        .map(|i| "*".repeat(i))
        .collect::<Vec<_>>()
        .join("\n");

    writeln!(writer, "{}", ans).unwrap();
}

// https://www.acmicpc.net/problem/2523
// 별 찍기 - 13
#[test]
fn test_solve2523() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "3".to_string(),
            want: "*
**
***
**
*"
            .to_string(),
        },
        TestData {
            s: "4".to_string(),
            want: "*
**
***
****
***
**
*"
            .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve2523(&mut reader, &mut writer);

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
