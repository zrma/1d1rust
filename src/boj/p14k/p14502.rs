use crate::read_values_as;
use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve14502(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (n, m) = read_values_as!(read_line(reader), usize, usize);
    let map = (0..n)
        .map(|_| {
            let s = read_line(reader);
            let mut iter = s.split_whitespace();
            (0..m)
                .map(|_| read_value(iter.next().unwrap().to_string()))
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let res = max_safe_area(n, m, map);
    write!(writer, "{}", res).expect("Failed to write");
}

fn max_safe_area(n: usize, m: usize, map: Vec<Vec<i32>>) -> i32 {
    build_wall_and_calculate_area(n, m, map, 0, 0, 0)
}

fn build_wall_and_calculate_area(
    n: usize,
    m: usize,
    mut map: Vec<Vec<i32>>,
    start_row: usize,
    walls_built: usize,
    max_area: i32,
) -> i32 {
    if walls_built == 3 {
        return std::cmp::max(max_area, calc_safe_area(n, m, &map));
    }

    let mut current_max_area = max_area;
    for row in start_row..n {
        for col in 0..m {
            if map[row][col] == 0 {
                map[row][col] = 1; // Build wall
                current_max_area = build_wall_and_calculate_area(
                    n,
                    m,
                    map.clone(),
                    row,
                    walls_built + 1,
                    current_max_area,
                );
                map[row][col] = 0; // Remove wall
            }
        }
    }
    current_max_area
}

fn calc_safe_area(n: usize, m: usize, map: &[Vec<i32>]) -> i32 {
    let mut visited = vec![vec![false; m]; n];
    let mut queue = std::collections::VecDeque::new();
    spread_virus(n, m, map, &mut visited, &mut queue);

    calculate_uninfected_area(n, m, map, &visited)
}

fn calculate_uninfected_area(n: usize, m: usize, map: &[Vec<i32>], visited: &[Vec<bool>]) -> i32 {
    let mut safe_area = 0;
    for i in 0..n {
        for j in 0..m {
            if map[i][j] == 0 && !visited[i][j] {
                safe_area += 1;
            }
        }
    }
    safe_area
}

fn spread_virus(
    n: usize,
    m: usize,
    map: &[Vec<i32>],
    visited: &mut [Vec<bool>],
    queue: &mut std::collections::VecDeque<(usize, usize)>,
) {
    for (i, row) in map.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == 2 {
                queue.push_back((i, j));
            }
        }
    }

    while let Some((y, x)) = queue.pop_front() {
        if visited[y][x] {
            continue;
        }
        visited[y][x] = true;

        if y > 0 && map[y - 1][x] == 0 {
            queue.push_back((y - 1, x));
        }
        if y < n - 1 && map[y + 1][x] == 0 {
            queue.push_back((y + 1, x));
        }
        if x > 0 && map[y][x - 1] == 0 {
            queue.push_back((y, x - 1));
        }
        if x < m - 1 && map[y][x + 1] == 0 {
            queue.push_back((y, x + 1));
        }
    }
}

// https://www.acmicpc.net/problem/14502
// 연구소
#[test]
fn test_solve14502() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "7 7
2 0 0 0 1 1 0
0 0 1 0 1 2 0
0 1 1 0 1 0 0
0 1 0 0 0 0 0
0 0 0 0 0 1 1
0 1 0 0 0 0 0
0 1 0 0 0 0 0"
                .to_string(),
            want: "27".to_string(),
        },
        TestData {
            s: "4 6
0 0 0 0 0 0
1 0 0 0 0 2
1 1 1 0 0 2
0 0 0 0 0 2"
                .to_string(),
            want: "9".to_string(),
        },
        TestData {
            s: "8 8
2 0 0 0 0 0 0 2
2 0 0 0 0 0 0 2
2 0 0 0 0 0 0 2
2 0 0 0 0 0 0 2
2 0 0 0 0 0 0 2
0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0"
                .to_string(),
            want: "3".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve14502(&mut reader, &mut writer);

        let got = String::from_utf8(writer).expect("Failed to convert writer to string");
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
