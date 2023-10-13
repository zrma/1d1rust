use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
pub(crate) fn solve1000(reader: &mut impl BufRead, writer: &mut impl Write) {
    let line = read_line(reader);

    let nums: Vec<i32> = line
        .split_whitespace()
        .map(|num_str| num_str.parse::<i32>().unwrap())
        .collect();

    let res = nums[0] + nums[1];

    write!(writer, "{}", res).unwrap();
}

// https://www.acmicpc.net/problem/1000
// A+B
#[test]
fn test_solve1000() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "1 2".to_string(),
            want: "3".to_string(),
        },
        TestData {
            s: "-1 -2".to_string(),
            want: "-3".to_string(),
        },
        TestData {
            s: "-1 2".to_string(),
            want: "1".to_string(),
        },
        TestData {
            s: "1 -2".to_string(),
            want: "-1".to_string(),
        },
        TestData {
            s: "0 0".to_string(),
            want: "0".to_string(),
        },
        TestData {
            s: "1 9".to_string(),
            want: "10".to_string(),
        },
        TestData {
            s: "9 1".to_string(),
            want: "10".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve1000(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
