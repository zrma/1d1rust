use crate::read_values;
use crate::utils::functions::try_next_pos;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve4963(reader: &mut impl BufRead, writer: &mut impl Write) {
    let mut ans = String::new();
    let mut line = String::new();

    while {
        line.clear();
        reader.read_line(&mut line).unwrap_or(0) > 0
    } {
        let (w, h) = read_values!(line.as_str(), usize, usize);
        if w == 0 && h == 0 {
            break;
        }
        let mut map = vec![vec![false; w]; h];
        for m in map.iter_mut() {
            line.clear();
            reader.read_line(&mut line).unwrap();
            *m = line.split_whitespace().map(|v| v == "1").collect();
        }
        ans.push_str(&format!("{}\n", count_islands(&map, w, h)));
    }

    write!(writer, "{}", ans).unwrap();
}

fn count_islands(map: &Vec<Vec<bool>>, w: usize, h: usize) -> i32 {
    let mut visited = vec![vec![false; map[0].len()]; map.len()];
    let mut count = 0;

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if !visited[y][x] && map[y][x] {
                visit(map, &mut visited, x, y, w, h);
                count += 1;
            }
        }
    }

    count
}

fn visit(
    map: &Vec<Vec<bool>>,
    visited: &mut Vec<Vec<bool>>,
    x: usize,
    y: usize,
    w: usize,
    h: usize,
) {
    if visited[y][x] || !map[y][x] {
        return;
    }

    visited[y][x] = true;

    for dy in -1..=1 {
        for dx in -1..=1 {
            if dy == 0 && dx == 0 {
                continue;
            }

            let (nx, ny) = match try_next_pos(w, h, x, y, dx, dy) {
                Some(v) => v,
                None => continue,
            };
            visit(map, visited, nx, ny, w, h);
        }
    }
}

// https://www.acmicpc.net/problem/4963
// 섬의 개수
#[test]
fn test_solve4963() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "1 1
0
2 2
0 1
1 0
3 2
1 1 1
1 1 1
5 4
1 0 1 0 0
1 0 0 0 0
1 0 1 0 1
1 0 0 1 0
5 4
1 1 1 0 1
1 0 1 0 1
1 0 1 0 1
1 0 1 1 1
5 5
1 0 1 0 1
0 0 0 0 0
1 0 1 0 1
0 0 0 0 0
1 0 1 0 1
0 0"
            .to_string(),
            want: "0
1
1
3
1
9
"
            .to_string(),
        },
        TestData {
            s: "5 5
1 0 1 0 1
0 0 0 0 0
1 0 1 0 1
0 0 0 0 0
1 0 1 0 1
0 0"
            .to_string(),
            want: "9
"
            .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let reader = &mut data.s.as_bytes();
        let mut writer = vec![];
        solve4963(reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
