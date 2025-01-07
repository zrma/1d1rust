use crate::utils::io::{read_line, read_n_values, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve17070(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: usize = read_value(read_line(reader));
    let mut house = vec![vec![0; n]; n];

    // 집의 상태 입력 받기
    for row in house.iter_mut().take(n) {
        *row = read_n_values(reader, n);
    }

    // dp[i][j][k]: (i,j) 위치에 k 방향으로 파이프가 놓인 경우의 수
    // k = 0: 가로, 1: 세로, 2: 대각선
    let mut dp = vec![vec![vec![0; 3]; n]; n];
    dp[0][1][0] = 1; // 초기 상태: (0,1)에 가로로 놓인 파이프

    for i in 0..n {
        for j in 0..n {
            if house[i][j] == 1 {
                continue;
            }

            // 가로 파이프 (k = 0)
            if j > 0 {
                // 이전 가로
                dp[i][j][0] += dp[i][j - 1][0];
                // 이전 대각선
                dp[i][j][0] += dp[i][j - 1][2];
            }

            // 세로 파이프 (k = 1)
            if i > 0 {
                // 이전 세로
                dp[i][j][1] += dp[i - 1][j][1];
                // 이전 대각선
                dp[i][j][1] += dp[i - 1][j][2];
            }

            // 대각선 파이프 (k = 2)
            if i > 0 && j > 0 && house[i - 1][j] == 0 && house[i][j - 1] == 0 {
                // 이전 가로
                dp[i][j][2] += dp[i - 1][j - 1][0];
                // 이전 세로
                dp[i][j][2] += dp[i - 1][j - 1][1];
                // 이전 대각선
                dp[i][j][2] += dp[i - 1][j - 1][2];
            }
        }
    }

    // 마지막 위치 (n-1, n-1)에 도달하는 모든 방향의 경우의 수 합
    let result = dp[n - 1][n - 1].iter().sum::<i32>();
    writeln!(writer, "{}", result).unwrap();
}

// https://www.acmicpc.net/problem/17070
// 파이프 옮기기 1
#[test]
fn test_solve17070() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "3
0 0 0
0 0 0
0 0 0"
                .to_string(),
            want: "1".to_string(),
        },
        TestData {
            s: "4
0 0 0 0
0 0 0 0
0 0 0 0
0 0 0 0"
                .to_string(),
            want: "3".to_string(),
        },
        TestData {
            s: "5
0 0 1 0 0
0 0 0 0 0
0 0 0 0 0
0 0 0 0 0
0 0 0 0 0"
                .to_string(),
            want: "0".to_string(),
        },
        TestData {
            s: "6
0 0 0 0 0 0
0 1 0 0 0 0
0 0 0 0 0 0
0 0 0 0 0 0
0 0 0 0 0 0
0 0 0 0 0 0"
                .to_string(),
            want: "13".to_string(),
        },
        TestData {
            s: "16
0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 1"
                .to_string(),
            want: "0".to_string(),
        },
        TestData {
            s: "16
0 0 0 0 0 1 0 0 0 0 0 0 0 0 0 0
0 1 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 1 0 0 0 0 0 0 0
0 0 0 0 0 1 0 0 0 0 0 0 0 0 0 0
0 0 0 0 1 0 0 0 0 1 0 0 1 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 1 1 0 0 1 1 0 0 0 0 0 1 0
0 0 0 0 0 0 0 1 0 0 0 1 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 1 0 0 0 0 0 0 0
0 0 0 0 0 0 1 0 0 0 0 1 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 1 0 0 0 1 0 0 0 1 0 1 0 0
0 0 0 0 1 0 0 0 0 0 0 0 0 0 0 0
0 1 0 0 0 0 0 0 0 0 0 0 0 0 1 1
0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0"
                .to_string(),
            want: "109".to_string(),
        },
        TestData {
            s: "16
0 0 0 0 0 0 0 0 0 1 0 0 0 0 0 1
0 1 0 1 0 1 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 1 0 0 0 0 0 0 0
0 0 0 0 0 0 1 0 0 0 0 0 0 1 1 0
0 0 0 0 0 0 0 0 0 0 0 0 0 1 0 0
0 0 0 0 0 0 0 0 0 0 1 0 0 0 0 0
0 0 1 0 0 0 0 0 0 0 0 1 0 0 0 0
0 0 0 0 0 0 0 0 1 1 0 0 0 1 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 1 0 0
0 0 0 0 0 0 0 0 0 0 0 0 1 1 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 1 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0"
                .to_string(),
            want: "664".to_string(),
        },
        TestData {
            s: "16
0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 1 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0"
                .to_string(),
            want: "1226203".to_string(),
        },
        TestData {
            s: "3
0 0 0
0 0 0
0 0 1"
                .to_string(),
            want: "0".to_string(),
        },
        TestData {
            s: "6
0 0 0 0 0 0
0 0 0 0 0 0
0 0 0 0 1 0
0 0 1 0 0 0
0 0 0 0 0 0
0 0 0 0 1 0"
                .to_string(),
            want: "1".to_string(),
        },
        TestData {
            s: "4
0 0 0 0
0 0 0 0
0 0 0 1
0 0 1 0"
                .to_string(),
            want: "0".to_string(),
        },
        TestData {
            s: "4
0 0 0 0
0 0 0 0
0 0 0 1
0 0 0 0"
                .to_string(),
            want: "0".to_string(),
        },
        TestData {
            s: "4
0 0 0 0
0 0 0 0
0 0 0 0
0 0 1 0"
                .to_string(),
            want: "2".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve17070(&mut reader, &mut writer);

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
