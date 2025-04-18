use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve1038(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: usize = read_value(read_line(reader));
    let mut nums: Vec<i64> = Vec::new();
    for mask in 1..(1 << 10) {
        let mut num: i64 = 0;
        for d in (0..10).rev() {
            if mask & (1 << d) != 0 {
                num = num * 10 + d as i64;
            }
        }
        nums.push(num);
    }
    nums.sort_unstable();
    if n < nums.len() {
        writeln!(writer, "{}", nums[n]).unwrap();
    } else {
        writeln!(writer, "-1").unwrap();
    }
}

// https://www.acmicpc.net/problem/1038
// 감소하는 수
#[test]
fn test_solve1038() {
    struct TestCase {
        s: String,
        want: String,
    }
    for (i, tc) in [
        TestCase {
            s: "18".to_string(),
            want: "42".to_string(),
        },
        TestCase {
            s: "0".to_string(),
            want: "0".to_string(),
        },
        TestCase {
            s: "500000".to_string(),
            want: "-1".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = tc.s.as_bytes();
        let mut writer = vec![];
        solve1038(&mut reader, &mut writer);
        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got.trim(), tc.want.trim(), "failed at {} with {}", i, tc.s);
    }
}
