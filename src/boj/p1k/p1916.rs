use crate::read_values_as;
use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve1916(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n = read_value(read_line(reader));
    let m = read_value(read_line(reader));

    let graph = read_graph(reader, n, m);

    let (start, end) = read_values_as!(read_line(reader), usize, usize);

    let ans = find_shortest_path(&graph, n, start, end);
    write!(writer, "{}", ans).expect("write! should work");
}

fn read_graph(reader: &mut impl BufRead, n: usize, m: usize) -> Vec<Vec<Way>> {
    let mut graph = vec![vec![]; n];
    for _ in 0..m {
        let (from, to, cost) = read_values_as!(read_line(reader), usize, usize, i32);
        graph[from - 1].push(Way {
            from: from - 1,
            to: to - 1,
            cost,
        });
    }
    graph
}

fn find_shortest_path(graph: &[Vec<Way>], n: usize, start: usize, end: usize) -> i32 {
    let mut dist = vec![i32::MAX; n];
    dist[start - 1] = 0;

    let mut queue = std::collections::BinaryHeap::new();
    queue.push(std::cmp::Reverse(Way {
        from: start - 1,
        to: start - 1,
        cost: 0,
    }));

    while let Some(std::cmp::Reverse(Way { to, cost, .. })) = queue.pop() {
        if dist[to] < cost {
            continue;
        }

        for &next_way in &graph[to] {
            let next_dist = cost + next_way.cost;
            if dist[next_way.to] > next_dist {
                dist[next_way.to] = next_dist;
                queue.push(std::cmp::Reverse(next_way.with_cost(next_dist)));
            }
        }
    }

    dist[end - 1]
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Way {
    from: usize,
    to: usize,
    cost: i32,
}

impl Way {
    fn with_cost(&self, cost: i32) -> Way {
        Way {
            from: self.from,
            to: self.to,
            cost,
        }
    }
}

// https://www.acmicpc.net/problem/1916
// 최소비용 구하기
#[test]
fn test_solve1916() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "5
8
1 2 2
1 3 3
1 4 1
1 5 10
2 4 2
3 4 1
3 5 1
4 5 3
1 5"
            .to_string(),
            want: "4".to_string(),
        },
        TestData {
            s: "5
8
1 2 2
1 3 3
1 4 1
1 5 10
2 4 1
3 4 1
3 5 2
4 5 3
1 5"
            .to_string(),
            want: "4".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve1916(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("writer should be a valid string");
        assert_eq!(
            got.trim(),
            data.want.trim(),
            "failed at {} with {}",
            i,
            data.s
        );
    }
}
