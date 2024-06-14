use crate::read_values_as;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve1182(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (n, s) = read_values_as!(read_line(reader), usize, i64);
    let nums: Vec<i64> = read_line(reader)
        .split_whitespace()
        .take(n)
        .map(|v| v.parse().unwrap())
        .collect();

    let mut ans = 0;
    choose_nums(&nums, 0, 0, s, &mut ans, false);

    write!(writer, "{}", ans).expect("Failed to write");
}

fn choose_nums(nums: &[i64], sum: i64, i: usize, s: i64, count: &mut i64, is_non_empty: bool) {
    if i == nums.len() {
        if is_non_empty && sum == s {
            *count += 1;
        }
        return;
    }

    choose_nums(nums, sum + nums[i], i + 1, s, count, true);
    choose_nums(nums, sum, i + 1, s, count, is_non_empty);
}

// https://www.acmicpc.net/problem/1182
// 부분수열의 합
#[test]
fn test_solve1182() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "5 0
-7 -3 -2 5 8"
                .to_string(),
            want: "1".to_string(),
        },
        TestData {
            s: "5 3
1 1 1 1 1"
                .to_string(),
            want: "10".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve1182(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
