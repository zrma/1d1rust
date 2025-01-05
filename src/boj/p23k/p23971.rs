use crate::read_values_as;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve23971(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (h, w, n, m) = read_values_as!(read_line(reader), u32, u32, u32, u32);

    let rows = (h + n) / (n + 1);
    let cols = (w + m) / (m + 1);
    let ans = rows * cols;

    writeln!(writer, "{}", ans).unwrap();
}

// https://www.acmicpc.net/problem/23971
// noinspection SpellCheckingInspection
// ZOAC 4
#[test]
fn test_solve23971() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "5 4 1 1".to_string(),
            want: "6".to_string(),
        },
        TestData {
            s: "4 4 1 1".to_string(),
            want: "4".to_string(),
        },
        TestData {
            s: "3 3 1 1".to_string(),
            want: "4".to_string(),
        },
        TestData {
            s: "4 3 1 1".to_string(),
            want: "4".to_string(),
        },
        TestData {
            s: "5 3 1 1".to_string(),
            want: "6".to_string(),
        },
        TestData {
            s: "5 4 2 2".to_string(),
            want: "4".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve23971(&mut reader, &mut writer);

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
