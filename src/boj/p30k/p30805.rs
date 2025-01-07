use crate::utils::io::{read_line, read_n_values, read_value};
use std::cmp::Ordering;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve30805(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: usize = read_value(read_line(reader));
    let a: Vec<usize> = read_n_values(reader, n);

    let m: usize = read_value(read_line(reader));
    let b: Vec<usize> = read_n_values(reader, m);

    // dp[i][j]: A[i..]와 B[j..]에서 얻을 수 있는 "사전 순으로 가장 나중" 공통 부분 수열
    let mut dp = vec![vec![Vec::new(); m + 1]; n + 1];

    // 뒤에서부터 채워 넣기
    for i in (0..n).rev() {
        for j in (0..m).rev() {
            let mut best = dp[i + 1][j].clone(); // A 쪽 요소를 버리는 경우
            let skip_b = &dp[i][j + 1]; // B 쪽 요소를 버리는 경우

            // skip_b vs best 비교하여 더 사전순으로 큰 쪽을 고름
            if lex_compare(skip_b, &best) == Ordering::Greater {
                best = skip_b.clone();
            }

            // 만약 A[i]와 B[j]가 같다면, 그 숫자를 포함하는 후보도 비교
            if a[i] == b[j] {
                let mut candidate = vec![a[i]];
                // 뒤쪽은 dp[i+1][j+1]
                candidate.extend_from_slice(&dp[i + 1][j + 1]);
                if lex_compare(&candidate, &best) == Ordering::Greater {
                    best = candidate;
                }
            }

            dp[i][j] = best;
        }
    }

    let ans = &dp[0][0];
    writeln!(writer, "{}", ans.len()).unwrap();
    if !ans.is_empty() {
        for (i, num) in ans.iter().enumerate() {
            if i > 0 {
                write!(writer, " ").unwrap();
            }
            write!(writer, "{}", num).unwrap();
        }
        writeln!(writer).unwrap();
    }
}

fn lex_compare(a: &[usize], b: &[usize]) -> Ordering {
    // "사전 순으로 누가 더 나중(더 큰가)?"를 판단하는 비교 함수
    for (x, y) in a.iter().zip(b.iter()) {
        match x.cmp(y) {
            Ordering::Equal => continue,
            ord => return ord,
        }
    }
    // 여기까지 왔다면, 둘 중 하나가 prefix. 더 긴 쪽이 사전 순으로 더 큼
    a.len().cmp(&b.len())
}

// https://www.acmicpc.net/problem/30805
// 사전 순 최대 공통 부분 수열
#[test]
fn test_solve30805() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "4
1 9 7 3
5
1 8 7 5 3"
                .to_string(),
            want: "2
7 3"
            .to_string(),
        },
        TestData {
            s: "2
100 100
2
100 100"
                .to_string(),
            want: "2
100 100"
                .to_string(),
        },
        TestData {
            s: "1
10
8
8 4 10 1 10 8 2 4"
                .to_string(),
            want: "1
10"
            .to_string(),
        },
        TestData {
            s: "10
9 8 7 6 5 1 2 3 4 5
8
1 3 5 7 9 6 5 4"
                .to_string(),
            want: "4
9 6 5 4"
                .to_string(),
        },
        TestData {
            s: "5
2 2 2 2 2
5
1 2 1 1 1"
                .to_string(),
            want: "1
2"
            .to_string(),
        },
        TestData {
            s: "8
4 2 9 6 8 9 10 9
5
6 7 6 9 8"
                .to_string(),
            want: "2
9 8"
            .to_string(),
        },
        TestData {
            s: "8
4 2 9 6 8 9 10 7
6
6 7 6 9 8 7"
                .to_string(),
            want: "3
9 8 7"
                .to_string(),
        },
        TestData {
            s: "5
1 2 8 4 9
5
8 4 1 2 7"
                .to_string(),
            want: "2
8 4"
            .to_string(),
        },
        TestData {
            s: "6
5 4 3 1 5 3
7
5 2 1 3 5 4 3"
                .to_string(),
            want: "3
5 5 3"
                .to_string(),
        },
        TestData {
            s: "7
5 4 3 1 5 4 3
7
5 2 1 3 5 4 3"
                .to_string(),
            want: "4
5 5 4 3"
                .to_string(),
        },
        TestData {
            s: "5
4 4 3 3 4
5
4 3 3 4 4"
                .to_string(),
            want: "3
4 4 4"
                .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve30805(&mut reader, &mut writer);

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
