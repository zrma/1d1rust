use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve3507(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: usize = read_value(read_line(reader));

    let ans = 199usize.saturating_sub(n);

    writeln!(writer, "{}", ans).unwrap();
}

// https://www.acmicpc.net/problem/3507
// Automated Telephone Exchange
#[test]
fn test_solve3507() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "100".to_string(),
            want: "99".to_string(), // 99, 1 | 98, 2 | ... | 2, 98 | 1, 99
        },
        TestData {
            s: "196".to_string(),
            want: "3".to_string(), // 99, 97 | 98, 98 | 97, 99
        },
        TestData {
            s: "197".to_string(),
            want: "2".to_string(), // 99, 98 | 98, 99
        },
        TestData {
            s: "198".to_string(),
            want: "1".to_string(), // 99, 99
        },
        TestData {
            s: "199".to_string(),
            want: "0".to_string(),
        },
        TestData {
            s: "239".to_string(),
            want: "0".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve3507(&mut reader, &mut writer);

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
