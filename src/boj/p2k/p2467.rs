use crate::utils::io::{read_line, read_n_values, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve2467(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: usize = read_value(read_line(reader));
    let arr: Vec<i32> = read_n_values(reader, n);

    let mut left = 0;
    let mut right = n - 1;
    let mut min_sum = i32::MAX;
    let mut ans = (arr[left], arr[right]);

    while left < right {
        let sum = arr[left] + arr[right];
        if sum.abs() < min_sum.abs() {
            min_sum = sum;
            ans = (arr[left], arr[right]);
        }

        match sum.cmp(&0) {
            std::cmp::Ordering::Less => left += 1,
            std::cmp::Ordering::Greater => right -= 1,
            std::cmp::Ordering::Equal => break,
        }
    }

    writeln!(writer, "{} {}", ans.0, ans.1).unwrap();
}

// https://www.acmicpc.net/problem/2467
// 용액
#[test]
fn test_solve2467() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "5
-99 -2 -1 4 98"
                .to_string(),
            want: "-99 98".to_string(),
        },
        TestData {
            s: "4
-100 -2 -1 103"
                .to_string(),
            want: "-100 103".to_string(),
        },
        TestData {
            s: "5
-100 -50 0 50 100"
                .to_string(),
            want: "-100 100".to_string(),
        },
        TestData {
            s: "3
-5 -4 -3"
                .to_string(),
            want: "-4 -3".to_string(),
        },
        TestData {
            s: "3
3 4 5"
                .to_string(),
            want: "3 4".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve2467(&mut reader, &mut writer);

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
