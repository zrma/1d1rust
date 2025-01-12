use crate::read_values_as;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve11281(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (n, m): (usize, usize) = read_values_as!(read_line(reader), usize, usize);

    let mut ts = TwoSat::new(n);

    // 각 절((x ∨ y))에 대해 add_clause
    for _ in 0..m {
        let (x, y): (i32, i32) = read_values_as!(read_line(reader), i32, i32);
        ts.add_clause(x, y);
    }

    // Kosaraju로 SCC를 구함
    ts.build_scc(); // 재귀 Kosaraju

    // satisfiable 판정
    if !ts.satisfiable() {
        // 해가 없으면 0 출력
        writeln!(writer, "0").unwrap();
    } else {
        // 해가 있으면 1 출력 + 실제 할당
        writeln!(writer, "1").unwrap();

        let assign = ts.assignment();
        for &val in &assign {
            write!(writer, "{} ", if val { 1 } else { 0 }).unwrap();
        }
        writeln!(writer).unwrap();
    }
}

/// 2-SAT 문제를 풀기 위한 구조체
///
/// - n: 변수의 개수 (각 변수 i에 대해, "i의 양(positive)"와 "i의 음(negative)" 정점을 2개 사용)
/// - graph, rev_graph: 정방향 / 역방향 그래프 인접 리스트
/// - scc_id: 각 정점(양/음)에 대한 SCC 번호 (build_scc 후에 할당)
/// - order, visited: Kosaraju용 임시 자료
struct TwoSat {
    n: usize,
    graph: Vec<Vec<usize>>,
    rev_graph: Vec<Vec<usize>>,
    scc_id: Vec<usize>,
    order: Vec<usize>, // 1단계 DFS 후의 postorder
    visited: Vec<bool>,
}

impl TwoSat {
    /// 새로 2-SAT 구조를 만든다.
    /// - 내부적으로 2*n개의 정점을 가진다. (각 변수에 대해 양/음 각각 1개씩)
    fn new(n: usize) -> Self {
        Self {
            n,
            graph: vec![Vec::new(); 2 * n],
            rev_graph: vec![Vec::new(); 2 * n],
            scc_id: vec![0; 2 * n],
            order: Vec::with_capacity(2 * n),
            visited: vec![false; 2 * n],
        }
    }

    /// (x ∨ y) 절 추가: (¬x -> y), (¬y -> x)
    /// x, y: ±1..±n
    fn add_clause(&mut self, x: i32, y: i32) {
        let xi = Self::to_index(x);
        let yi = Self::to_index(y);
        let not_xi = xi ^ 1;
        let not_yi = yi ^ 1;

        // (¬x -> y)
        self.graph[not_xi].push(yi);
        self.rev_graph[yi].push(not_xi);

        // (¬y -> x)
        self.graph[not_yi].push(xi);
        self.rev_graph[xi].push(not_yi);
    }

    /// 내부적으로 사용하는: x(±1..±n)를 정점 인덱스로
    fn to_index(x: i32) -> usize {
        let absx = x.unsigned_abs() as usize;
        let base = 2 * (absx - 1);
        if x > 0 {
            base
        } else {
            base + 1
        }
    }

    fn build_scc(&mut self) {
        // 1) 순방향 DFS(postorder)
        self.visited.fill(false);
        for u in 0..2 * self.n {
            if !self.visited[u] {
                dfs_forward(u, &mut self.visited, &self.graph, &mut self.order);
            }
        }

        // 2) 역방향 DFS
        self.visited.fill(false);
        let mut scc_idx = 0;
        for &u in self.order.iter().rev() {
            if !self.visited[u] {
                dfs_backward(
                    u,
                    &mut self.visited,
                    &self.rev_graph,
                    &mut self.scc_id,
                    scc_idx,
                );
                scc_idx += 1;
            }
        }
    }

    /// 해가 있는지 여부: 같은 변수의 양/음 정점이 같은 SCC이면 불가능
    fn satisfiable(&self) -> bool {
        for i in 0..self.n {
            if self.scc_id[2 * i] == self.scc_id[2 * i + 1] {
                return false;
            }
        }
        true
    }

    /// SCC 번호가 더 큰 쪽을 true로 두는 전형적 2-SAT 할당
    fn assignment(&self) -> Vec<bool> {
        let mut ans = vec![false; self.n];
        for (i, val) in ans.iter_mut().enumerate().take(self.n) {
            *val = self.scc_id[2 * i] > self.scc_id[2 * i + 1];
        }
        ans
    }
}

/// 재귀 DFS(순방향) - postorder 채우기
fn dfs_forward(u: usize, visited: &mut [bool], graph: &[Vec<usize>], order: &mut Vec<usize>) {
    visited[u] = true;
    for &v in &graph[u] {
        if !visited[v] {
            dfs_forward(v, visited, graph, order);
        }
    }
    order.push(u);
}

/// 재귀 DFS(역방향) - SCC 번호 할당
fn dfs_backward(
    u: usize,
    visited: &mut [bool],
    rev_graph: &[Vec<usize>],
    scc_id: &mut [usize],
    scc_idx: usize,
) {
    visited[u] = true;
    scc_id[u] = scc_idx;
    for &v in &rev_graph[u] {
        if !visited[v] {
            dfs_backward(v, visited, rev_graph, scc_id, scc_idx);
        }
    }
}

// https://www.acmicpc.net/problem/11281
// 2-SAT - 4
#[test]
fn test_solve11281() {
    struct TestCase {
        s: String,
        want: String,
    }
    for (i, data) in vec![
        TestCase {
            s: "3 4
-1 2
-2 3
1 3
3 2"
            .to_string(),
            want: "1
1 1 1"
                .to_string(),
        },
        TestCase {
            s: "1 2
1 1
-1 -1"
                .to_string(),
            want: "0".to_string(),
        },
        TestCase {
            s: "5 7
1 -5
-5 4
4 -2
1 2
2 -3
-1 -4
-1 3"
                .to_string(),
            want: "1
0 1 0 1 0"
                .to_string(),
        },
        TestCase {
            s: "3 5
-2 -1
-1 -1
2 3
-3 1
1 3"
            .to_string(),
            want: "0".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve11281(&mut reader, &mut writer);

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
