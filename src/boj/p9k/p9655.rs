use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve9655(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: usize = read_value(read_line(reader));
    let ans = if n % 2 == 1 { "SK" } else { "CY" };
    writeln!(writer, "{}", ans).unwrap();
}

#[allow(dead_code)]
fn solve9655_dp(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: usize = read_value(read_line(reader));
    let mut dp = vec![false; n + 1];
    dp[1] = true;

    for i in 2..=n {
        let prev_turn = !dp[i - 1];
        let three_turn_away = if i >= 3 { !dp[i - 3] } else { false };
        dp[i] = prev_turn || three_turn_away;
    }

    let ans = if dp[n] { "SK" } else { "CY" };
    writeln!(writer, "{}", ans).unwrap();
}

// https://www.acmicpc.net/problem/9655
// 돌 게임
#[test]
fn test_solve9655() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "1".to_string(),
            want: "SK".to_string(),
        },
        TestData {
            s: "2".to_string(),
            want: "CY".to_string(),
        },
        TestData {
            s: "3".to_string(),
            want: "SK".to_string(),
        },
        TestData {
            s: "4".to_string(),
            want: "CY".to_string(),
        },
        TestData {
            s: "5".to_string(),
            want: "SK".to_string(),
        },
        TestData {
            s: "6".to_string(),
            want: "CY".to_string(),
        },
        TestData {
            s: "7".to_string(),
            want: "SK".to_string(),
        },
        TestData {
            s: "8".to_string(),
            want: "CY".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        {
            let mut reader = &mut data.s.as_bytes();
            let mut writer = Vec::new();
            solve9655(&mut reader, &mut writer);

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
            solve9655_dp(&mut reader, &mut writer);

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
