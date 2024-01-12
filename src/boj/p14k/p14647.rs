use crate::read_values;
use crate::utils::io::read_line;
use std::io::{BufRead, Write};

#[allow(dead_code)]
fn solve14647(reader: &mut impl BufRead, writer: &mut impl Write) {
    let (n, m) = read_values!(read_line(reader), usize, usize);
    let mut grid = (0..n)
        .map(|_| {
            read_line(reader)
                .split_whitespace()
                .take(m)
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let max_row = find_max_index(&grid, n, m, true);
    let max_col = find_max_index(&grid, n, m, false);

    if max_row.1 > max_col.1 {
        clear_row(&mut grid, max_row.0);
    } else {
        clear_column(&mut grid, max_col.0);
    }

    let bird_count = count_birds(&grid);
    write!(writer, "{}", bird_count).unwrap();
}

fn find_max_index(grid: &[Vec<String>], n: usize, m: usize, is_row: bool) -> (usize, usize) {
    let mut max_idx = 0;
    let mut max_count = 0;
    for i in 0..(if is_row { n } else { m }) {
        let count = (0..(if is_row { m } else { n }))
            .map(|j| {
                grid[if is_row { i } else { j }][if is_row { j } else { i }]
                    .chars()
                    .filter(|&ch| ch == '9')
                    .count()
            })
            .sum::<usize>();
        if count > max_count {
            max_count = count;
            max_idx = i;
        }
    }
    (max_idx, max_count)
}

fn clear_row(grid: &mut Vec<Vec<String>>, row_idx: usize) {
    for cell in &mut grid[row_idx] {
        cell.clear();
    }
}

fn clear_column(grid: &mut Vec<Vec<String>>, col_idx: usize) {
    for row in grid {
        row[col_idx].clear();
    }
}

fn count_birds(grid: &[Vec<String>]) -> usize {
    grid.iter()
        .map(|row| {
            row.iter()
                .map(|x| x.chars().filter(|&ch| ch == '9').count())
                .sum::<usize>()
        })
        .sum()
}

// https://www.acmicpc.net/problem/14647
// 준오는 조류혐오야!!
#[test]
fn test1_14647() {
    struct TestData {
        s: String,
        want: String,
    }
    for (i, data) in [
        TestData {
            s: "3 4
        1 2 3 9
        4 5 9 6
        9 7 8 9"
                .to_string(),
            want: "2".to_string(),
        },
        TestData {
            s: "4 4
11 12 19 14
99 39 14 90
13 47 81 99
32 72 29 66"
                .to_string(),
            want: "4".to_string(),
        },
    ]
    .iter()
    .enumerate()
    {
        let mut reader = data.s.as_bytes();
        let mut writer = vec![];
        solve14647(&mut reader, &mut writer);

        let got = String::from_utf8(writer).unwrap();
        assert_eq!(got, data.want, "failed at {} with {}", i, data.s);
    }
}
