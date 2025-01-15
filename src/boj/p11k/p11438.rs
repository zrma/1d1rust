use crate::read_values_as;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

const MAX_LOG: usize = 17; // 2^17 > 100,000

#[allow(dead_code)]
fn solve11438(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: usize = read_line(reader).trim().parse().unwrap();

    // 그래프 구성
    let mut graph = vec![Vec::new(); n + 1];
    for _ in 1..n {
        let (u, v): (usize, usize) = read_values_as!(read_line(reader), usize, usize);
        graph[u].push(v);
        graph[v].push(u);
    }

    // 각 노드의 깊이와 2^k번째 조상을 저장
    let mut depth = vec![0; n + 1];
    let mut parent = vec![vec![0; n + 1]; MAX_LOG];

    // DFS로 깊이와 첫 번째 부모를 계산
    dfs(1, 0, 0, &graph, &mut depth, &mut parent[0]);

    // 2^k번째 조상 테이블 구성
    build_lca_table(n, &mut parent);

    // 쿼리 처리
    let m: usize = read_line(reader).trim().parse().unwrap();
    for _ in 0..m {
        let (u, v): (usize, usize) = read_values_as!(read_line(reader), usize, usize);
        let lca = find_lca(u, v, &depth, &parent);
        writeln!(writer, "{}", lca).unwrap();
    }
}

fn dfs(
    curr: usize,
    prev: usize,
    d: usize,
    graph: &[Vec<usize>],
    depth: &mut [usize],
    parent: &mut [usize],
) {
    depth[curr] = d;
    parent[curr] = prev;

    for &next in &graph[curr] {
        if next != prev {
            dfs(next, curr, d + 1, graph, depth, parent);
        }
    }
}

fn build_lca_table(n: usize, parent: &mut [Vec<usize>]) {
    for k in 1..MAX_LOG {
        for node in 1..=n {
            parent[k][node] = parent[k - 1][parent[k - 1][node]];
        }
    }
}

fn find_lca(mut u: usize, mut v: usize, depth: &[usize], parent: &[Vec<usize>]) -> usize {
    // u를 더 깊은 노드로 설정
    if depth[u] < depth[v] {
        std::mem::swap(&mut u, &mut v);
    }

    // 깊이 맞추기
    let diff = depth[u] - depth[v];
    for (k, _) in (0..MAX_LOG).enumerate().take(MAX_LOG) {
        if (diff >> k) & 1 == 1 {
            u = parent[k][u];
        }
    }

    if u == v {
        return u;
    }

    // 공통 조상 찾기
    for k in (0..MAX_LOG).rev() {
        if parent[k][u] != parent[k][v] {
            u = parent[k][u];
            v = parent[k][v];
        }
    }

    parent[0][u]
}

// https://www.acmicpc.net/problem/11438
// LCA 2
#[test]
fn test_solve11438() {
    struct TestCase {
        s: String,
        want: String,
    }
    for (i, data) in vec![
        TestCase {
            s: "15
1 2
1 3
2 4
3 7
6 2
3 8
4 9
2 5
5 11
7 13
10 4
11 15
12 5
14 7
6
6 11
10 9
2 6
7 6
8 13
8 15"
                .to_string(),
            want: "2
4
2
1
3
1"
            .to_string(),
        },
        TestCase {
            s: "5
1 5
5 4
4 3
3 2
1
1 2"
            .to_string(),
            want: "1".to_string(),
        },
        TestCase {
            s: "14
1 2
2 3
3 4
4 5
5 6
6 7
7 8
8 9
4 10
10 11
11 12
12 13
13 14
1
9 14"
                .to_string(),
            want: "4".to_string(),
        },
        TestCase {
            s: "3
3 1
3 2
3
1 2
1 3
2 3"
            .to_string(),
            want: "1
1
3"
            .to_string(),
        },
        TestCase {
            s: "30
1  2
2  3
3  4
4  5
5  6
6  7
7  8
8  9
9  10
10 11
11 12
12 13
13 14
14 15
15 16
16 17
17 18
18 19
19 20
20 21
21 22
22 23
23 24
24 25
25 26
26 27
27 28
28 29
29 30
1
6 11"
                .to_string(),
            want: "6".to_string(),
        },
        TestCase {
            s: "22
1 2
1 3
3 4
4 5
5 6
6 7
7 8
8 9
9 10
10 11
11 12
12 13
13 14
14 15
15 16
16 17
17 18
18 19
19 20
20 21
21 22
2
22 21
21 22"
                .to_string(),
            want: "21
21"
            .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve11438(&mut reader, &mut writer);

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
