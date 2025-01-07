use crate::utils::functions::try_next_pos;
use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve2468(reader: &mut impl BufRead, writer: &mut impl Write) {
    let map_size: usize = read_value(read_line(reader));

    let heights_map: Vec<Vec<i32>> = (0..map_size)
        .map(|_| {
            read_line(reader)
                .split_whitespace()
                .take(map_size)
                .map(|s| s.parse().unwrap())
                .collect()
        })
        .collect();

    let max_height = heights_map
        .iter()
        .flat_map(|row| row.iter())
        .max()
        .cloned()
        .unwrap_or(0);

    let mut max_safe_areas = 0;
    for water_level in 0..=max_height {
        let mut visited = vec![vec![false; map_size]; map_size];
        let mut current_safe_areas = 0;

        for y in 0..map_size {
            for x in 0..map_size {
                if !visited[y][x] && heights_map[y][x] > water_level {
                    flood_fill(&heights_map, &mut visited, water_level, y, x, map_size);
                    current_safe_areas += 1;
                }
            }
        }
        max_safe_areas = max_safe_areas.max(current_safe_areas);
    }

    writeln!(writer, "{}", max_safe_areas).unwrap();
}

fn flood_fill(
    heights_map: &[Vec<i32>],
    visited: &mut Vec<Vec<bool>>,
    water_level: i32,
    y: usize,
    x: usize,
    map_size: usize,
) {
    if heights_map[y][x] <= water_level || visited[y][x] {
        return;
    }

    visited[y][x] = true;

    let adjacent_positions = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    for &(dy, dx) in adjacent_positions.iter() {
        let (nx, ny) = match try_next_pos(map_size, map_size, x, y, dx, dy) {
            Some(v) => v,
            None => continue,
        };

        flood_fill(heights_map, visited, water_level, ny, nx, map_size);
    }
}

// https://www.acmicpc.net/problem/2468
// 안전 영역
#[test]
fn test_solve2468() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "5
6 8 2 6 2
3 2 3 4 6
6 7 3 3 2
7 2 5 3 6
8 9 5 2 7"
                .to_string(),
            want: "5".to_string(),
        },
        TestData {
            s: "7
9 9 9 9 9 9 9
9 2 1 2 1 2 9
9 1 8 7 8 1 9
9 2 7 9 7 2 9
9 1 8 7 8 1 9
9 2 1 2 1 2 9
9 9 9 9 9 9 9"
                .to_string(),
            want: "6".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve2468(&mut reader, &mut writer);

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
