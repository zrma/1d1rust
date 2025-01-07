use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve9659(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: usize = read_value(read_line(reader));
    let ans = if n % 2 == 0 { "CY" } else { "SK" };
    writeln!(writer, "{}", ans).unwrap();
}

#[allow(dead_code)]
fn solve9659_dp(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: usize = read_value(read_line(reader));

    let mut dp = vec![false; n + 1];
    if n >= 1 {
        dp[1] = true;
    }
    if n >= 3 {
        dp[3] = true;
    }

    for i in 4..=n {
        dp[i] = !dp[i - 1] || !dp[i - 3];
    }

    let ans = if dp[n] { "SK" } else { "CY" };
    writeln!(writer, "{}", ans).unwrap();
}

// https://www.acmicpc.net/problem/9659
// 돌 게임 5
#[test]
fn test_solve9659() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "1".to_string(),
            want: "SK".to_string(), // 1 (o)
        },
        TestData {
            s: "2".to_string(),
            want: "CY".to_string(), // 1 + 1 (x)
        },
        TestData {
            s: "3".to_string(),
            want: "SK".to_string(), // 1 + 2 (o) | 3 (o)
        },
        TestData {
            s: "4".to_string(),
            want: "CY".to_string(), // 1 + 3 (x) | 3 + 1 (x)
        },
        TestData {
            s: "5".to_string(),
            want: "SK".to_string(), // 1 + 4 (o) | 3 + 2 (x)
        },
        TestData {
            s: "6".to_string(),
            want: "CY".to_string(), // 1 + 5 (x) | 3 + 3 (x)
        },
        TestData {
            s: "7".to_string(),
            want: "SK".to_string(), // 1 + 6 (o) | 3 + 4 (o)
        },
        TestData {
            s: "8".to_string(),
            want: "CY".to_string(), // 1 + 7 (x) | 3 + 5 (x)
        },
    ]
    .iter()
    .enumerate()
    {
        {
            let mut reader = data.s.as_bytes();
            let mut writer = vec![];
            solve9659(&mut reader, &mut writer);

            let got = String::from_utf8(writer).unwrap();
            assert_eq!(
                got.trim(),
                data.want.trim(),
                "failed at {} with {}",
                i,
                data.s
            );
        }

        {
            let mut reader = data.s.as_bytes();
            let mut writer = vec![];
            solve9659_dp(&mut reader, &mut writer);

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
}
