use crate::read_values_as;
use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve2533(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n: usize = read_value(read_line(reader));
    let mut graph = vec![vec![]; n + 1];

    // 그래프 입력
    for _ in 0..n - 1 {
        let (u, v): (usize, usize) = read_values_as!(read_line(reader), usize, usize);
        graph[u].push(v);
        graph[v].push(u);
    }

    // dp[u][0]: u가 얼리 어답터가 아닐 때 서브트리 최소 얼리 어답터 수
    // dp[u][1]: u가 얼리 어답터일 때 서브트리 최소 얼리 어답터 수
    let mut dp = vec![[0, 0]; n + 1];
    let mut visited = vec![false; n + 1];
    let mut parent = vec![0; n + 1];
    let mut post = Vec::with_capacity(n);

    // 1번 노드를 루트로 삼아 스택 기반 DFS(후위 순회 순서 만들기)
    let root = 1;
    let mut stack = vec![root];
    visited[root] = true;

    while let Some(u) = stack.pop() {
        post.push(u);
        for &v in &graph[u] {
            if !visited[v] {
                visited[v] = true;
                parent[v] = u;
                stack.push(v);
            }
        }
    }

    // 후위 순회 결과를 뒤에서부터 처리하여 dp 테이블 채우기
    for &u in post.iter().rev() {
        dp[u][0] = 0; // u가 얼리 어답터가 아닐 때
        dp[u][1] = 1; // u가 얼리 어답터일 때(자기 자신 포함하므로 1)

        for &v in &graph[u] {
            if v == parent[u] {
                continue;
            }
            // u가 얼리 어답터가 아니면, 자식 v는 반드시 얼리 어답터여야 함
            dp[u][0] += dp[v][1];
            // u가 얼리 어답터라면, 자식 v는 얼리 어답터든 아니든 최소인 쪽 선택
            dp[u][1] += dp[v][0].min(dp[v][1]);
        }
    }

    // 루트에 대해 얼리 어답터이거나 아니거나 중 최솟값
    let answer = dp[root][0].min(dp[root][1]);
    writeln!(writer, "{}", answer).unwrap();
}

// https://www.acmicpc.net/problem/2533
// 사회망 서비스(SNS)
#[test]
fn test_solve2533() {
    struct TestCase {
        s: String,
        want: String,
    }
    for (i, data) in vec![
        TestCase {
            s: "8
1 2
1 3
1 4
2 5
2 6
4 7
4 8"
            .to_string(),
            want: "3".to_string(),
        },
        TestCase {
            s: "9
1 2
1 3
2 4
3 5
3 6
4 7
4 8
4 9"
            .to_string(),
            want: "3".to_string(),
        },
        TestCase {
            s: "22
1 4
2 1
3 1
5 2
6 2
19 5
20 5
6 21
6 22
19 11
19 12
20 13
14 20
15 21
16 21
17 22
22 18
7 11
8 11
9 12
12 10"
                .to_string(),
            want: "8".to_string(),
        },
        TestCase {
            s: "8
8 5
8 2
6 2
4 3
1 2
1 7
4 2"
            .to_string(),
            want: "4".to_string(),
        },
        TestCase {
            s: "16
1 2
1 3
1 4
1 5
1 6
2 7
3 8
4 9
5 10
8 11
9 12
9 13
9 14
9 15
9 16"
                .to_string(),
            want: "5".to_string(),
        },
        TestCase {
            s: "8
1 2
1 3
1 4
2 5
2 6
4 7
4 8"
            .to_string(),
            want: "3".to_string(),
        },
        TestCase {
            s: "9
1 2
1 3
2 4
3 5
3 6
4 7
4 8
4 9"
            .to_string(),
            want: "3".to_string(),
        },
        TestCase {
            s: "10
1 2
2 3
2 4
4 6
4 5
6 7
6 8
8 9
8 10"
                .to_string(),
            want: "4".to_string(),
        },
        TestCase {
            s: "6
1 2
2 3
2 4
4 5
4 6"
            .to_string(),
            want: "2".to_string(),
        },
        TestCase {
            s: "11
1 2
2 3
2 4
2 5
3 6
6 7
4 8
4 9
5 10
1 11"
                .to_string(),
            want: "5".to_string(),
        },
        TestCase {
            s: "5
1 5
2 5
3 5
4 5"
            .to_string(),
            want: "1".to_string(),
        },
        TestCase {
            s: "9
1 2
2 3
3 4
4 5
2 6
6 7
7 8
1 9"
            .to_string(),
            want: "4".to_string(),
        },
        TestCase {
            s: "5
1 2
2 3
3 4
4 5"
            .to_string(),
            want: "2".to_string(),
        },
        TestCase {
            s: "11
1 2
2 3
2 4
1 5
5 6
6 7
6 8
1 9
9 10
9 11"
                .to_string(),
            want: "4".to_string(),
        },
        TestCase {
            s: "10
1 2
1 3
2 4
3 5
3 6
4 7
4 8
5 9
5 10"
                .to_string(),
            want: "4".to_string(),
        },
        TestCase {
            s: "10
1 2
2 3
2 4
4 6
4 5
6 7
6 8
8 9
8 10"
                .to_string(),
            want: "4".to_string(),
        },
        TestCase {
            s: "6
1 2
2 3
2 4
4 5
4 6"
            .to_string(),
            want: "2".to_string(),
        },
        TestCase {
            s: "11
1 2
2 3
2 4
2 5
3 6
6 7
4 8
4 9
5 10
1 11"
                .to_string(),
            want: "5".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve2533(&mut reader, &mut writer);

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
