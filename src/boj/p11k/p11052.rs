use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve11052(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n = read_value(read_line(reader));
    let mut prices: Vec<i32> = read_line(reader)
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    prices.insert(0, 0);

    let mut dp = vec![0; n + 1];
    for i in 1..=n {
        for j in 1..=i {
            dp[i] = dp[i].max(dp[i - j] + prices[j]);
        }
    }

    write!(writer, "{}", dp[n]).expect("Failed to write");
}

// https://www.acmicpc.net/problem/11052
// 카드 구매하기
#[test]
fn test_solve11052() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "4
1 5 6 7"
                .to_string(),
            want: "10".to_string(),
        },
        TestData {
            s: "5
10 9 8 7 6"
                .to_string(),
            want: "50".to_string(),
        },
        TestData {
            s: "10
1 1 2 3 5 8 13 21 34 55"
                .to_string(),
            want: "55".to_string(),
        },
        TestData {
            s: "10
5 10 11 12 13 30 35 40 45 47"
                .to_string(),
            want: "50".to_string(),
        },
        TestData {
            s: "4
5 2 8 10"
                .to_string(),
            want: "20".to_string(),
        },
        TestData {
            s: "4
3 5 15 16"
                .to_string(),
            want: "18".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve11052(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
