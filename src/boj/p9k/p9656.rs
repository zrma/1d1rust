use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve9656(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: usize = read_value(read_line(reader));
    let ans = if n % 2 == 0 { "SK" } else { "CY" };
    write!(writer, "{}", ans).expect("write! should work");
}

#[allow(dead_code)]
fn solve9656_dp(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: usize = read_value(read_line(reader));

    let mut dp = vec![false; n + 1];
    if n >= 2 {
        dp[2] = true;
    }

    for i in 4..=n {
        dp[i] = !dp[i - 1] || !dp[i - 3];
    }

    let ans = if dp[n] { "SK" } else { "CY" };
    write!(writer, "{}", ans).expect("write! should work");
}

// https://www.acmicpc.net/problem/9656
// 돌 게임 2
#[test]
fn test_solve9656() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "1".to_string(),
            want: "CY".to_string(),
        },
        TestData {
            s: "2".to_string(),
            want: "SK".to_string(),
        },
        TestData {
            s: "3".to_string(),
            want: "CY".to_string(),
        },
        TestData {
            s: "4".to_string(),
            want: "SK".to_string(),
        },
        TestData {
            s: "5".to_string(),
            want: "CY".to_string(),
        },
        TestData {
            s: "6".to_string(),
            want: "SK".to_string(),
        },
        TestData {
            s: "7".to_string(),
            want: "CY".to_string(),
        },
        TestData {
            s: "8".to_string(),
            want: "SK".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        {
            let mut reader = &mut data.s.as_bytes();
            let mut writer = Vec::new();
            solve9656(&mut reader, &mut writer);

            let got = String::from_utf8(writer).expect("writer should be a valid string");
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
            solve9656_dp(&mut reader, &mut writer);

            let got = String::from_utf8(writer).expect("writer should be a valid string");
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
