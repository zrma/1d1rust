use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve9658(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: usize = read_value(read_line(reader));
    let ans = if n % 7 == 1 || n % 7 == 3 { "CY" } else { "SK" };
    writeln!(writer, "{}", ans).unwrap();
}

#[allow(dead_code)]
fn solve9658_dp(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: usize = read_value(read_line(reader));

    let mut dp = vec![false; n + 1];
    if n >= 2 {
        dp[2] = true;
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

// https://www.acmicpc.net/problem/9658
// 돌 게임 4
#[test]
fn test_solve9658() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "1".to_string(),
            want: "CY".to_string(), // 1 (x)
        },
        TestData {
            s: "2".to_string(),
            want: "SK".to_string(), // 1 + 1 (o)
        },
        TestData {
            s: "3".to_string(),
            want: "CY".to_string(), // 1 + 2 (x), 3 (x)
        },
        TestData {
            s: "4".to_string(),
            want: "SK".to_string(), // 1 + 3 (o) | 3 + 1 (o)
        },
        TestData {
            s: "5".to_string(),
            want: "SK".to_string(), // 1 + 4 (x) | 3 + 2 (x) | 4 + 1 (o)
        },
        TestData {
            s: "6".to_string(),
            want: "SK".to_string(), // 1 + 5 (x) | 3 + 3 (o) | 4 + 2 (o)
        },
        TestData {
            s: "7".to_string(),
            want: "SK".to_string(), // 1 + 6 (x) | 3 + 4 (x) | 4 + 3 (o)
        },
        TestData {
            s: "8".to_string(),
            want: "CY".to_string(), // 1 + 7 (x) | 3 + 5 (x) | 4 + 4 (x)
        },
        TestData {
            s: "9".to_string(),
            want: "SK".to_string(), // 1 + 8 (o) | 3 + 6 (x) | 4 + 5 (x)
        },
        TestData {
            s: "10".to_string(),
            want: "CY".to_string(), // 1 + 9 (x) | 3 + 7 (x) | 4 + 6 (x)
        },
        TestData {
            s: "11".to_string(),
            want: "SK".to_string(), // 1 + 10 (o) | 3 + 8 (o) | 4 + 7 (x)
        },
    ]
    .iter()
    .enumerate()
    {
        {
            let mut reader = &mut data.s.as_bytes();
            let mut writer = Vec::new();
            solve9658(&mut reader, &mut writer);

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
            solve9658_dp(&mut reader, &mut writer);

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
