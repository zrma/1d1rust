use crate::read_values_as;
use crate::utils::functions::try_next_pos;
use crate::utils::io::read_line;
use std::collections::HashSet;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve14466(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (fields_size, num_cows, num_roads) =
        read_values_as!(read_line(reader), usize, usize, usize);
    let blocked_paths = (0..num_roads)
        .map(|_| {
            let (r1, c1, r2, c2) = read_values_as!(read_line(reader), usize, usize, usize, usize);
            normalize_coordinates(Point::new(r1 - 1, c1 - 1), Point::new(r2 - 1, c2 - 1))
        })
        .collect::<HashSet<_>>();
    let cows = (0..num_cows)
        .map(|_| {
            let (r, c) = read_values_as!(read_line(reader), usize, usize);
            Point::new(r - 1, c - 1)
        })
        .collect::<Vec<_>>();

    let mut ans = 0;
    for (i, &cow) in cows.iter().enumerate() {
        let mut visited = vec![vec![false; fields_size]; fields_size];
        dfs(fields_size, &blocked_paths, &mut visited, cow);
        ans += cows
            .iter()
            .skip(i + 1)
            .filter(|&&p| !visited[p.row][p.col])
            .count();
    }

    writeln!(writer, "{}", ans).unwrap();
}

fn dfs(
    fields_size: usize,
    blocked_paths: &HashSet<(Point, Point)>,
    visited: &mut Vec<Vec<bool>>,
    curr_pos: Point,
) {
    visited[curr_pos.row][curr_pos.col] = true;

    for &(dr, dc) in &[(0, -1), (0, 1), (-1, 0), (1, 0)] {
        if let Some((next_row, next_col)) =
            try_next_pos(fields_size, fields_size, curr_pos.row, curr_pos.col, dr, dc)
        {
            let next_pos = Point::new(next_row, next_col);
            if !visited[next_row][next_col]
                && !blocked_paths.contains(&normalize_coordinates(curr_pos, next_pos))
            {
                dfs(fields_size, blocked_paths, visited, next_pos);
            }
        }
    }
}

fn normalize_coordinates(p1: Point, p2: Point) -> (Point, Point) {
    if p1 < p2 {
        (p1, p2)
    } else {
        (p2, p1)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Point {
    row: usize,
    col: usize,
}

impl Point {
    fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

// https://www.acmicpc.net/problem/14466
// 소가 길을 건너간 이유 6
#[test]
fn test_solve14466() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "3 3 3
2 2 2 3
3 3 3 2
3 3 2 3
3 3
2 2
2 3"
            .to_string(),
            want: "2".to_string(),
        },
        TestData {
            s: "3 3 3
1 1 1 2
1 1 2 1
2 1 2 2
3 3
2 2
2 3"
            .to_string(),
            want: "0".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve14466(&mut reader, &mut writer);

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
