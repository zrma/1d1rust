use crate::utils::io::{read_line, read_value};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve2468(reader: &mut impl BufRead, writer: &mut impl Write) {
    let map_size: usize = read_value(read_line(reader));

    let heights_map = (0..map_size)
        .map(|_| {
            read_line(reader)
                .split_whitespace()
                .take(map_size)
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

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

    write!(writer, "{}", max_safe_areas).unwrap();
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

    let is_in_map =
        |y: isize, x: isize| y >= 0 && y < map_size as isize && x >= 0 && x < map_size as isize;

    for &(dy, dx) in adjacent_positions.iter() {
        let new_y = y as isize + dy;
        let new_x = x as isize + dx;

        if is_in_map(new_y, new_x) {
            flood_fill(
                heights_map,
                visited,
                water_level,
                new_y as usize,
                new_x as usize,
                map_size,
            );
        }
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
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
