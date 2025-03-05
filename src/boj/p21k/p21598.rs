use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve21598(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: i32 = read_value(read_line(reader));

    for _ in 0..n {
        writeln!(writer, "SciComLove").unwrap();
    }
}

// https://www.acmicpc.net/problem/21598
// SciComLove
#[test]
fn test_solve21598() {
    struct TestCase {
        s: String,
        want: String,
    }
    for (i, tc) in [
        TestCase {
            s: "1".to_string(),
            want: "SciComLove
"
            .to_string(),
        },
        TestCase {
            s: "2".to_string(),
            want: "SciComLove
SciComLove
"
            .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = tc.s.as_bytes();
        let mut writer = vec![];
        solve21598(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, tc.want, "failed at {} with {}", i, tc.s);
    }
}
