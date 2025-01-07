use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve9657(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: usize = read_value(read_line(reader));
    let ans = if n % 7 == 0 || n % 7 == 2 { "CY" } else { "SK" };
    writeln!(writer, "{}", ans).unwrap();
}

#[allow(dead_code)]
fn solve9657_dp(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: usize = read_value(read_line(reader));

    let mut dp = vec![false; n + 1];
    if n >= 1 {
        dp[1] = true;
    }
    if n >= 3 {
        dp[3] = true;
    }
    if n >= 4 {
        dp[4] = true;
    }

    for i in 5..=n {
        dp[i] = !dp[i - 1] || !dp[i - 3] || !dp[i - 4];
    }

    let ans = if dp[n] { "SK" } else { "CY" };
    writeln!(writer, "{}", ans).unwrap();
}

// https://www.acmicpc.net/problem/9657
// 돌 게임 3
#[test]
fn test_solve9657() {
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
            want: "SK".to_string(), // 3 (o)
        },
        TestData {
            s: "4".to_string(),
            want: "SK".to_string(), // 4 (o)
        },
        TestData {
            s: "5".to_string(),
            want: "SK".to_string(), // 1 + 4 (x) | 3 + 2 (o)
        },
        TestData {
            s: "6".to_string(),
            want: "SK".to_string(), // 1 + 5 (x) | 3 + 3 (x) | 4 + 2 (o)
        },
        TestData {
            s: "7".to_string(),
            want: "CY".to_string(), // 1 + 6 (x) | 3 + 4 (x) | 4 + 3 (x)
        },
        TestData {
            s: "8".to_string(),
            want: "SK".to_string(), // 1 + 7 (o) | 3 + 5 (x) | 4 + 4 (x)
        },
        TestData {
            s: "9".to_string(),
            want: "CY".to_string(), // 1 + 8 (x) | 3 + 6 (x) | 4 + 5 (x)
        },
        TestData {
            s: "10".to_string(),
            want: "SK".to_string(), // 1 + 9 (o) | 3 + 7 (o) | 4 + 6 (x)
        },
        TestData {
            s: "11".to_string(),
            want: "SK".to_string(), // 1 + 10 (x) | 3 + 8 (x) | 4 + 7 (o)
        },
    ]
    .iter()
    .enumerate()
    {
        {
            let mut reader = &mut data.s.as_bytes();
            let mut writer = Vec::new();
            solve9657(&mut reader, &mut writer);

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
            let mut reader = &mut data.s.as_bytes();
            let mut writer = Vec::new();
            solve9657_dp(&mut reader, &mut writer);

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
