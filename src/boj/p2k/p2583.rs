use crate::read_values_as;
use crate::utils::functions::try_next_pos;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve2583(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (row, col, rects) = read_input(reader);

    let mut table = vec![vec![false; col]; row];
    fill_table(&mut table, &rects);

    let res = find_areas(&mut table);

    writeln!(writer, "{}", res.len()).unwrap();
    for n in res {
        write!(writer, "{} ", n).unwrap();
    }
}

fn find_areas(table: &mut [Vec<bool>]) -> Vec<usize> {
    let (row, col) = (table.len(), table[0].len());
    let mut res = vec![];

    for y in 0..row {
        for x in 0..col {
            if !table[y][x] {
                let cnt = bfs(table, x, y);
                res.push(cnt);
            }
        }
    }

    res.sort_unstable();
    res
}

fn read_input(reader: &mut impl BufRead) -> (usize, usize, Vec<Vec<usize>>) {
    let (row, col, n) = read_values_as!(read_line(reader), usize, usize, usize);

    let rects: Vec<Vec<usize>> = (0..n)
        .map(|_| {
            read_line(reader)
                .split_whitespace()
                .map(|num_str| num_str.parse().unwrap())
                .collect()
        })
        .collect();

    (row, col, rects)
}

fn fill_table(table: &mut [Vec<bool>], rects: &[Vec<usize>]) {
    for rect in rects {
        (rect[1]..rect[3]).for_each(|y| {
            (rect[0]..rect[2]).for_each(|x| {
                table[y][x] = true;
            })
        });
    }
}

fn bfs(table: &mut [Vec<bool>], x: usize, y: usize) -> usize {
    struct Pos {
        x: usize,
        y: usize,
    }

    let mut cnt = 0;
    let mut queue = vec![Pos { x, y }];

    while let Some(pos) = queue.pop() {
        if table[pos.y][pos.x] {
            continue;
        }
        cnt += 1;
        table[pos.y][pos.x] = true;

        let deltas = [(0, -1), (0, 1), (-1, 0), (1, 0)];
        for (dx, dy) in deltas.iter() {
            let (nx, ny) = match try_next_pos(table[0].len(), table.len(), pos.x, pos.y, *dx, *dy) {
                Some(v) => v,
                None => continue,
            };

            queue.push(Pos { x: nx, y: ny });
        }
    }

    cnt
}

// https://www.acmicpc.net/problem/2583
// 영역 구하기
#[test]
fn test_solve2583() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "5 7 3
0 2 4 4
1 1 2 5
4 0 6 2"
                .to_string(),
            want: "3
1 7 13 "
                .to_string(),
        },
        TestData {
            s: "100 100 0".to_string(),
            want: "1
10000 "
                .to_string(),
        },
        TestData {
            s: "100 100 1
0 0 1 1"
                .to_string(),
            want: "1
9999 "
                .to_string(),
        },
        TestData {
            s: "100 100 1
1 0 2 100"
                .to_string(),
            want: "2
100 9800 "
                .to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve2583(&mut reader, &mut writer);

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
