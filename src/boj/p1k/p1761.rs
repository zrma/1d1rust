use crate::read_values_as;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};
use std::mem::swap;

#[allow(dead_code)]
fn solve1761(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: usize = read_line(reader).parse().unwrap();

    // log2(N) ~ 16
    const MAX_LOG: usize = 16;

    let mut tree = TreeInfo {
        graph: vec![Vec::new(); n + 1],
        weights: vec![Vec::new(); n + 1],
        depth: vec![0; n + 1],
        dist: vec![0; n + 1],
        parent: vec![vec![0; n + 1]; MAX_LOG + 1],
    };

    for _ in 0..n - 1 {
        let (u, v, w): (usize, usize, i32) = read_values_as!(read_line(reader), usize, usize, i32);
        tree.graph[u].push(v);
        tree.graph[v].push(u);
        tree.weights[u].push(w);
        tree.weights[v].push(w);
    }

    // (1) DFS로 depth, dist, parent[0][child] 계산
    //     편의상 루트를 1번 노드로
    dfs(1, 0, 0, &mut tree);

    // (2) parent 테이블 채우기
    build_lca_table(&mut tree);

    // (3) 쿼리 처리
    let m: usize = read_line(reader).parse().unwrap();
    for _ in 0..m {
        let (u, v): (usize, usize) = read_values_as!(read_line(reader), usize, usize);
        let lca_node = lca(u, v, &tree.depth, &tree.parent);
        let distance = tree.dist[u] + tree.dist[v] - 2 * tree.dist[lca_node];
        writeln!(writer, "{}", distance).unwrap();
    }
}

struct TreeInfo {
    graph: Vec<Vec<usize>>,
    weights: Vec<Vec<i32>>,
    depth: Vec<usize>,
    dist: Vec<i32>,
    // parent[k][v]: v의 2^k번째 조상
    parent: Vec<Vec<usize>>,
}

// DFS를 이용해 깊이(depth), dist(루트~노드 거리), parent[0][child] 설정
fn dfs(current: usize, par: usize, d: usize, tree: &mut TreeInfo) {
    tree.depth[current] = d;
    tree.parent[0][current] = par;

    let next_nodes = tree.graph[current].clone();
    let next_weights = tree.weights[current].clone();

    for (i, next) in next_nodes.into_iter().enumerate() {
        if next == par {
            continue;
        }
        tree.dist[next] = tree.dist[current] + next_weights[i];
        dfs(next, current, d + 1, tree);
    }
}

// parent[k][v] = parent[k-1][ parent[k-1][v] ]
fn build_lca_table(tree: &mut TreeInfo) {
    let n = tree.depth.len() - 1; // 노드 개수
    let max_log = tree.parent.len(); // MAX_LOG + 1

    for k in 1..max_log {
        for v in 1..=n {
            let pv = tree.parent[k - 1][v];
            // pv의 2^(k-1)번째 조상을 다시 찾아서 연결
            tree.parent[k][v] = tree.parent[k - 1][pv];
        }
    }
}

// O(log N)으로 LCA 구하기
fn lca(mut u: usize, mut v: usize, depth: &[usize], parent: &[Vec<usize>]) -> usize {
    if depth[u] < depth[v] {
        swap(&mut u, &mut v);
    }
    // 1) 깊이 차를 먼저 맞추기
    let diff = depth[u] - depth[v];
    let max_log = parent.len();
    for (k, _) in parent.iter().enumerate().take(max_log) {
        if (diff & (1 << k)) != 0 {
            u = parent[k][u];
        }
    }
    // 이미 같다면 u(또는 v)가 LCA
    if u == v {
        return u;
    }
    // 2) 위에서부터 내려오며 (가능한 한) 부모가 다를 때 점프
    //    결국 u, v의 바로 부모가 같아지는 지점 직전에 멈춤
    for k in (0..max_log).rev() {
        if parent[k][u] != parent[k][v] {
            u = parent[k][u];
            v = parent[k][v];
        }
    }
    // 마지막으로 한 칸 올리면 LCA
    parent[0][u]
}

// https://www.acmicpc.net/problem/1761
// 정점들의 거리
#[test]
fn test_solve1761() {
    struct TestCase {
        s: String,
        want: String,
    }
    for (i, data) in vec![
        TestCase {
            s: "7
1 6 13
6 3 9
3 5 7
4 1 3
2 4 20
4 7 2
3
1 6
1 4
2 6"
            .to_string(),
            want: "13
3
36"
            .to_string(),
        },
        TestCase {
            s: "28
1 2 19
1 3 68
2 4 27
2 5 17
2 6 29
3 7 71
4 8 38
4 9 13
5 10 4
7 11 81
8 12 5
9 13 75
10 14 70
10 15 68
11 16 7
11 17 86
12 18 23
12 19 47
14 20 26
15 21 92
17 22 55
18 23 20
19 24 49
20 25 2
20 26 95
21 27 37
22 28 4
4
17 8
7 18
3 28
27 12"
                .to_string(),
            want: "390
251
297
288"
            .to_string(),
        },
        TestCase {
            s: "4
1 2 10
3 4 20
2 4 30
1
1 3"
            .to_string(),
            want: "60".to_string(),
        },
        TestCase {
            s: "15
1 2 1
1 3 1
2 4 1
3 7 1
6 2 1
3 8 1
4 9 1
2 5 1
5 11 1
7 13 1
10 4 1
11 15 1
12 5 1
14 7 1
6
6 11
10 9
2 6
7 6
8 13
8 15"
                .to_string(),
            want: "3
2
1
4
3
6"
            .to_string(),
        },
        TestCase {
            s: "4
2 1 2
2 3 3
4 2 1
1
2 4"
            .to_string(),
            want: "1".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve1761(&mut reader, &mut writer);

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
