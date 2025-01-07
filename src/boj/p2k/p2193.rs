use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve2193(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: usize = read_value(read_line(reader));

    let mut dp = vec![vec![0u128; 2]; n + 1];
    dp[1][1] = 1;
    for i in 2..=n {
        dp[i][0] = dp[i - 1][0] + dp[i - 1][1];
        dp[i][1] = dp[i - 1][0];
    }

    writeln!(writer, "{}", dp[n][0] + dp[n][1]).unwrap();
}

// https://www.acmicpc.net/problem/2193
// 이친수
#[test]
fn test_solve2193() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "3".to_string(),
            want: "2".to_string(),
        },
        TestData {
            s: "4".to_string(),
            want: "3".to_string(),
        },
        TestData {
            s: "5".to_string(),
            want: "5".to_string(),
        },
        TestData {
            s: "6".to_string(),
            want: "8".to_string(),
        },
        TestData {
            s: "7".to_string(),
            want: "13".to_string(),
        },
        TestData {
            s: "8".to_string(),
            want: "21".to_string(),
        },
        TestData {
            s: "9".to_string(),
            want: "34".to_string(),
        },
        TestData {
            s: "10".to_string(),
            want: "55".to_string(),
        },
        TestData {
            s: "90".to_string(),
            want: "2880067194370816120".to_string(),
        },
        TestData {
            s: "100".to_string(),
            want: "354224848179261915075".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve2193(&mut reader, &mut writer);

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
