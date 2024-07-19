use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};
use std::iter::successors;

#[allow(dead_code)]
fn solve14920(reader: &mut impl BufRead, writer: &mut impl Write) {
    let start_num: u32 = read_value(read_line(reader));

    let ans = successors(Some(start_num), |&current| match (current, current % 2) {
        (1, _) => None,
        (_, 0) => Some(current / 2),
        (_, _) => Some(3 * current + 1),
    })
    .count();

    write!(writer, "{}", ans).expect("Failed to write");
}

// https://www.acmicpc.net/problem/14920
// 3n+1 수열
#[test]
fn test_solve14920() {
    struct TestData {
        s: String,
        want: String,
    }

    for (i, data) in [
        TestData {
            s: "26".to_string(),
            want: "11".to_string(),
        },
        TestData {
            s: "7".to_string(),
            want: "17".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve14920(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
