use crate::utils::io::{read_line, read_value};
use std::collections::BinaryHeap;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve1414(reader: &mut impl BufRead, writer: &mut impl Write) {
    let n = read_value(read_line(reader));
    let mut parents = (0..n).collect::<Vec<_>>();
    let mut total = 0;
    let mut pq = BinaryHeap::new();

    for i in 0..n {
        for (j, ch) in read_line(reader).chars().enumerate() {
            if let Some(value) = char_to_value(ch) {
                total += value;
                pq.push(Node::new(i, j, value));
            }
        }
    }

    let mut included_nodes = 1;
    let mut mst_weight = 0;
    while let Some(node) = pq.pop() {
        if union_find(&mut parents, node.to, node.from) {
            included_nodes += 1;
            mst_weight += node.value;
        }
    }

    if included_nodes != n {
        write!(writer, "-1").expect("Failed to write");
    } else {
        write!(writer, "{}", total - mst_weight).expect("Failed to write");
    }
}

fn char_to_value(ch: char) -> Option<usize> {
    match ch {
        'a'..='z' => Some(ch as usize - 'a' as usize + 1),
        'A'..='Z' => Some(ch as usize - 'A' as usize + 27),
        _ => None,
    }
}

fn union_find(parents: &mut Vec<usize>, x: usize, y: usize) -> bool {
    let (rx, ry) = (find(parents, x), find(parents, y));
    if rx != ry {
        parents[rx.max(ry)] = rx.min(ry);
        true
    } else {
        false
    }
}

fn find(parents: &mut Vec<usize>, x: usize) -> usize {
    if parents[x] != x {
        parents[x] = find(parents, parents[x]);
    }
    parents[x]
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Node {
    from: usize,
    to: usize,
    value: usize,
}

impl Node {
    fn new(from: usize, to: usize, value: usize) -> Self {
        Self { from, to, value }
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.value.cmp(&self.value)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.value.cmp(&self.value))
    }
}

// https://www.acmicpc.net/problem/1414
// noinspection SpellCheckingInspection
// 불우이웃돕기
#[test]
fn test_solve1414() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "3
abc
def
ghi"
            .to_string(),
            want: "40".to_string(),
        },
        TestData {
            s: "2
a0
0b"
            .to_string(),
            want: "-1".to_string(),
        },
        TestData {
            s: "4
0X00
00Y0
0000
00Z0"
                .to_string(),
            want: "0".to_string(),
        },
        TestData {
            s: "2
Az
aZ"
            .to_string(),
            want: "105".to_string(),
        },
        TestData {
            s: "4
0top
c0od
er0o
pen0"
                .to_string(),
            want: "134".to_string(),
        },
        TestData {
            s: "1
a"
            .to_string(),
            want: "1".to_string(),
        },
        TestData {
            s: "1
0"
            .to_string(),
            want: "0".to_string(),
        },
        TestData {
            s: "4
aa00
aa00
00aa
00aa"
                .to_string(),
            want: "-1".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve1414(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
