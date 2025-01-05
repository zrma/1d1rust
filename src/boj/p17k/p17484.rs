use crate::read_values_as;
use crate::utils::io::{read_line, read_n_values};
use std::io::{BufRead, Write};

const INF: i32 = 1_000_000;

#[allow(dead_code)]
fn solve17484(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (n, m) = read_values_as!(read_line(reader), usize, usize);

    let mut grid = vec![vec![0; m]; n];
    grid.iter_mut().for_each(|row| {
        *row = read_n_values::<i32>(reader, m);
    });

    // dp[i][j][k]: i행 j열에 도착했을 때, 마지막 이동 방향이 k인 경우의 최소 비용
    // k: 0 = 왼쪽 대각선(좌하), 1 = 아래, 2 = 오른쪽 대각선(우하)
    let mut dp = vec![vec![vec![INF; 3]; m]; n];

    // 첫 행은 이전 이동 방향 제약이 없으므로 모두 초기화
    for j in 0..m {
        for k in 0..3 {
            dp[0][j][k] = grid[0][j];
        }
    }

    for i in 0..n - 1 {
        for j in 0..m {
            for k in 0..3 {
                if dp[i][j][k] == INF {
                    continue;
                }
                let current_cost = dp[i][j][k];

                // 다음 행으로 내려갈 때 가능한 세 방향을 각각 시도하되,
                // 이전 방향 k와 다른 방향으로만 전이한다.

                // 왼쪽 대각선 (0)
                if k != 0 && j > 0 {
                    let next_cost = current_cost + grid[i + 1][j - 1];
                    dp[i + 1][j - 1][0] = dp[i + 1][j - 1][0].min(next_cost);
                }

                // 아래 (1)
                if k != 1 {
                    let next_cost = current_cost + grid[i + 1][j];
                    dp[i + 1][j][1] = dp[i + 1][j][1].min(next_cost);
                }

                // 오른쪽 대각선 (2)
                if k != 2 && j + 1 < m {
                    let next_cost = current_cost + grid[i + 1][j + 1];
                    dp[i + 1][j + 1][2] = dp[i + 1][j + 1][2].min(next_cost);
                }
            }
        }
    }

    let mut min_fuel = INF;
    for j in 0..m {
        for k in 0..3 {
            if dp[n - 1][j][k] < min_fuel {
                min_fuel = dp[n - 1][j][k];
            }
        }
    }

    writeln!(writer, "{}", min_fuel).unwrap();
}

// https://www.acmicpc.net/problem/17484
// 진우의 달 여행 (Small)
#[test]
fn test_solve17484() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "6 4
5 8 5 1
3 5 8 4
9 77 65 5
2 1 5 2
5 98 1 5
4 95 67 58"
                .to_string(),
            want: "29".to_string(),
        },
        TestData {
            s: "3 3
1 2 3
4 5 6
7 8 9"
                .to_string(),
            want: "13".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve17484(&mut reader, &mut writer);

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
