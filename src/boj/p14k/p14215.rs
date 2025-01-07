use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve14215(reader: &mut impl BufRead, writer: &mut impl Write) {
    let mut nums = read_nums(reader);

    nums.sort_unstable();

    let res = if nums[0] + nums[1] > nums[2] {
        nums[0] + nums[1] + nums[2]
    } else {
        (nums[0] + nums[1]) * 2 - 1
    };

    writeln!(writer, "{}", res).unwrap();
}

fn read_nums(reader: &mut impl BufRead) -> Vec<usize> {
    read_line(reader)
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

// https://www.acmicpc.net/problem/14215
// 세 막대
#[test]
fn test_solve14215() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "1 2 3".to_string(),
            want: "5".to_string(),
        },
        TestData {
            s: "2 2 2".to_string(),
            want: "6".to_string(),
        },
        TestData {
            s: "3 2 1".to_string(),
            want: "5".to_string(),
        },
        TestData {
            s: "3 3 3".to_string(),
            want: "9".to_string(),
        },
        TestData {
            s: "1 100 1".to_string(),
            want: "3".to_string(),
        },
        TestData {
            s: "41 64 16".to_string(),
            want: "113".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve14215(&mut reader, &mut writer);

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
