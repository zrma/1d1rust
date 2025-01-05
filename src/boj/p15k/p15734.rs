use crate::read_values_as;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve15734(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (left, right, extra) = read_values_as!(read_line(reader), u32, u32, u32);
    let (mut left, mut right, mut extra) = (left, right, extra);

    let diff = right.abs_diff(left);
    if extra >= diff {
        extra -= diff;
        let max_value = left.max(right);
        left = max_value + extra / 2;
        right = max_value + extra / 2;
    } else if left < right {
        left += extra;
    } else {
        right += extra;
    }

    let ans = left.min(right) * 2;
    writeln!(writer, "{}", ans).unwrap();
}

// https://www.acmicpc.net/problem/15734
// 명장 남정훈
#[test]
fn test_solve15734() {
    struct TestData {
        s: String,
        want: String,
    }

    for (i, data) in [
        TestData {
            s: "1 5 2".to_string(),
            want: "6".to_string(),
        },
        TestData {
            s: "7 7 7".to_string(),
            want: "20".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve15734(&mut reader, &mut writer);

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
