use crate::read_values;
use crate::utils::io::{read_line, read_map};
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve2638(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (rows, cols) = read_values!(read_line(reader), usize, usize);

    let mut map = read_map(reader, rows, cols);

    let mut air = Vec::new();
    air.push((0, 0));

    let mut cheese = Vec::new();
    map.iter().enumerate().for_each(|(y, row)| {
        row.iter().enumerate().for_each(|(x, &cell)| {
            if cell == 1 {
                cheese.push((x, y));
            }
        })
    });

    let mut ans = 0;
    while !cheese.is_empty() {
        ans += 1;
        fill_air(&mut map, &mut air);
        melt_cheese(&mut map, &mut cheese, &mut air);
    }

    write!(writer, "{}", ans).unwrap();
}

fn fill_air(map: &mut Vec<Vec<usize>>, air: &mut Vec<(usize, usize)>) {
    while let Some((x, y)) = air.pop() {
        map[y][x] = 2;
        for (dx, dy) in DIRECTIONS.iter() {
            let (nx, ny) = ((x as i32 + dx) as usize, (y as i32 + dy) as usize);
            if is_valid(map, nx, ny) && map[ny][nx] == 0 {
                air.push((nx, ny));
            }
        }
    }
}

fn melt_cheese(
    map: &mut Vec<Vec<usize>>,
    cheese: &mut Vec<(usize, usize)>,
    air: &mut Vec<(usize, usize)>,
) {
    let mut new_cheese = Vec::new();
    while let Some((x, y)) = cheese.pop() {
        if count_adjacent_air(map, x, y) >= 2 {
            map[y][x] = 0;
            air.push((x, y));
        } else {
            new_cheese.push((x, y));
        }
    }
    *cheese = new_cheese;
}

fn count_adjacent_air(map: &Vec<Vec<usize>>, x: usize, y: usize) -> usize {
    DIRECTIONS
        .iter()
        .filter(|&&(dx, dy)| {
            let (nx, ny) = ((x as i32 + dx) as usize, (y as i32 + dy) as usize);
            is_valid(map, nx, ny) && map[ny][nx] == 2
        })
        .count()
}

fn is_valid(map: &Vec<Vec<usize>>, x: usize, y: usize) -> bool {
    x < map[0].len() && y < map.len()
}

const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

// https://www.acmicpc.net/problem/2638
// 치즈
#[test]
fn test_solve2638() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "8 9
0 0 0 0 0 0 0 0 0
0 0 0 1 1 0 0 0 0
0 0 0 1 1 0 1 1 0
0 0 1 1 1 1 1 1 0
0 0 1 1 1 1 1 0 0
0 0 1 1 0 1 1 0 0
0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0"
                .to_string(),
            want: "4".to_string(),
        },
        TestData {
            s: "5 5
0 0 0 0 0
0 1 1 1 0
0 1 1 1 0
0 1 1 1 0
0 0 0 0 0"
                .to_string(),
            want: "3".to_string(),
        },
        TestData {
            s: "7 7
0 0 0 0 0 0 0
0 1 1 1 1 1 0
0 1 1 1 1 1 0
0 1 1 1 1 1 0
0 1 1 1 1 1 0
0 1 1 1 1 1 0
0 0 0 0 0 0 0"
                .to_string(),
            want: "5".to_string(),
        },
        TestData {
            s: "7 7
0 0 0 0 0 0 0
0 1 1 1 1 1 0
0 1 1 1 1 1 0
0 1 1 0 1 1 0
0 1 1 1 1 1 0
0 1 1 1 1 1 0
0 0 0 0 0 0 0"
                .to_string(),
            want: "4".to_string(),
        },
        TestData {
            s: "7 7
0 0 0 0 0 0 0
0 1 1 1 1 1 0
0 1 1 1 1 1 0
0 1 0 1 0 1 0
0 1 1 1 1 1 0
0 1 1 1 1 1 0
0 0 0 0 0 0 0"
                .to_string(),
            want: "4".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve2638(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
