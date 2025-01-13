use crate::read_values_as;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

const MOD: i64 = 1_000_003;

#[allow(dead_code)]
fn solve1533(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (n, s, e, t): (usize, usize, usize, i64) =
        read_values_as!(read_line(reader), usize, usize, usize, i64);

    // 각 정점을 5개로 분할하여 큰 행렬을 만듦
    let size = n * 5;
    let mut matrix = vec![vec![0i64; size]; size];

    // 입력 그래프를 행렬로 변환
    for i in 0..n {
        let line = read_line(reader);
        for (j, c) in line.chars().enumerate() {
            let weight = c.to_digit(10).unwrap() as i64;
            if weight > 0 {
                // i번 정점의 0번 분할에서 j번 정점의 (weight-1)번 분할로 연결
                matrix[i * 5][j * 5 + (weight - 1) as usize] = 1;
            }
        }
        // 각 정점의 내부 분할 연결
        for j in 0..4 {
            matrix[i * 5 + j + 1][i * 5 + j] = 1;
        }
    }

    // 행렬의 t제곱 계산
    let result = matrix_power(&matrix, t);

    // s에서 e로 가는 경로 수 출력 (0번 분할끼리의 연결만 고려)
    writeln!(writer, "{}", result[(s - 1) * 5][(e - 1) * 5]).unwrap();
}

// 행렬 곱셈
fn matrix_multiply(a: &[Vec<i64>], b: &[Vec<i64>]) -> Vec<Vec<i64>> {
    let n = a.len();
    let mut result = vec![vec![0; n]; n];
    for i in 0..n {
        for (k, row) in b.iter().enumerate().take(n) {
            let aik = a[i][k];
            for (j, &val) in row.iter().enumerate().take(n) {
                result[i][j] = (result[i][j] + (aik * val) % MOD) % MOD;
            }
        }
    }
    result
}

// 행렬의 거듭제곱 (분할 정복)
fn matrix_power(matrix: &[Vec<i64>], mut power: i64) -> Vec<Vec<i64>> {
    let n = matrix.len();
    let mut result = vec![vec![0; n]; n];
    let mut base = matrix.to_vec();

    // 단위 행렬로 초기화
    for (i, row) in result.iter_mut().enumerate().take(n) {
        row[i] = 1;
    }

    // 분할 정복으로 거듭제곱 계산
    while power > 0 {
        if power & 1 == 1 {
            result = matrix_multiply(&result, &base);
        }
        base = matrix_multiply(&base, &base);
        power >>= 1;
    }

    result
}

// https://www.acmicpc.net/problem/1533
// 길의 개수
#[test]
fn test_solve1533() {
    struct TestCase {
        s: String,
        want: String,
    }
    for (i, data) in vec![
        TestCase {
            s: "3 1 3 5
012
201
120"
            .to_string(),
            want: "8".to_string(),
        },
        TestCase {
            s: "4 1 3 9
0544
1012
2304
2100"
                .to_string(),
            want: "2".to_string(),
        },
        TestCase {
            s: "3 1 2 2
013
100
200"
            .to_string(),
            want: "0".to_string(),
        },
        TestCase {
            s: "3 1 2 3
013
100
200"
            .to_string(),
            want: "1".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve1533(&mut reader, &mut writer);

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
