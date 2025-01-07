use crate::read_values_as;
use crate::utils::io::{read_line, read_n_values};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve14938(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (n, m, r) = read_values_as!(read_line(reader), usize, usize, usize);
    let items: Vec<i32> = read_n_values(reader, n);

    // 매우 큰 값(무한대 역할), 거리 비교용
    const INF: usize = 1_000_000_000;

    // 거리 정보를 담을 2차원 배열 (인덱스 편의를 위해 i-1 처리)
    let mut dist = vec![vec![INF; n]; n];
    for (i, row) in dist.iter_mut().enumerate().take(n) {
        row[i] = 0;
    }

    // r개의 간선 정보 읽기 (양방향)
    for _ in 0..r {
        let (a, b, l) = read_values_as!(read_line(reader), usize, usize, usize);
        // 1-based → 0-based
        dist[a - 1][b - 1] = dist[a - 1][b - 1].min(l);
        dist[b - 1][a - 1] = dist[b - 1][a - 1].min(l);
    }

    // 플로이드–워셜로 모든 쌍 최단 거리 계산
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if dist[i][j] > dist[i][k] + dist[k][j] {
                    dist[i][j] = dist[i][k] + dist[k][j];
                }
            }
        }
    }

    // 각 지점마다 "수색 범위 m 이하"로 갈 수 있는 지역의 아이템 합을 계산 후 최댓값 찾기
    let mut answer = 0;
    for distances in dist.iter().take(n) {
        let sum_items = distances
            .iter()
            .enumerate()
            .take(n)
            .filter(|(_, &d)| d <= m)
            .map(|(i, _)| items[i])
            .sum();
        answer = answer.max(sum_items);
    }

    writeln!(writer, "{}", answer).unwrap();
}

// https://www.acmicpc.net/problem/14938
// 서강그라운드
#[test]
fn test_solve14938() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "5 5 4
5 7 8 2 3
1 4 5
5 2 4
3 2 3
1 2 3"
                .to_string(),
            want: "23".to_string(),
        },
        TestData {
            s: "4 2 3
1 2 3 4
1 2 2
2 3 2
3 4 1"
                .to_string(),
            // 수색 범위가 2이므로, 예를 들어 3번 지점에서 (2,3,4번)은 거리가 (2,0,1)이므로 모두 포함 → 2+3+4=9가 최대
            want: "9".to_string(),
        },
        TestData {
            s: "5 1 4
1 1 1 1 1
1 2 1
2 3 1
3 4 1
4 5 1"
                .to_string(),
            // 수색 범위가 1이므로, 최대 3개 구역(본인 + 양 옆)까지만 가능 → 최대 합 3
            want: "3".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve14938(&mut reader, &mut writer);

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
