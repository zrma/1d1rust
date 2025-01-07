use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve2921(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: i64 = read_value(read_line(reader));

    let ans: i64 = (0..=n).flat_map(|i| (i..=n).map(move |j| i + j)).sum();

    writeln!(writer, "{}", ans).unwrap();
}

// https://www.acmicpc.net/problem/2921
// 도미노
#[test]
fn test_solve2921() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "2".to_string(),
            want: "12".to_string(),
        },
        TestData {
            s: "3".to_string(),
            want: "30".to_string(),
        },
        TestData {
            s: "15".to_string(),
            want: "2040".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve2921(&mut reader, &mut writer);

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
